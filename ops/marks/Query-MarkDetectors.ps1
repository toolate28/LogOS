# Query-MarkDetectors.ps1 — D1/D2/D4/D6/D7 read-only probes (Category B until live on origin)
# Usage:
#   pwsh -File ops/marks/Query-MarkDetectors.ps1
#   pwsh -File ops/marks/Query-MarkDetectors.ps1 -Remote origin -Ref main
#   pwsh -File ops/marks/Query-MarkDetectors.ps1 -CertHead HEAD   # D7 preflight before countersign

param(
    [string]$Remote = "origin",
    [string]$Ref = "main",
    [string]$LedgerPath = "ops/marks/MARKS.jsonl",
    # VERIFY survey head for D7 preflight (default: HEAD). When countersigns exist, also checked from Mark-Cert-Head.
    [string]$CertHead = "HEAD"
)

$ErrorActionPreference = "Stop"
Set-Location (git rev-parse --show-toplevel 2>$null)
if (-not $?) { Write-Error "Not inside a git work tree." }

function Get-TrailerMap([string]$Commit) {
    $raw = git log -1 $Commit --format="%(trailers:unfold,only)" 2>$null
    $map = @{}
    foreach ($line in ($raw -split "`n")) {
        if ($line -match '^\s*([^:]+):\s*(.*)$') {
            $map[$Matches[1].Trim()] = $Matches[2].Trim()
        }
    }
    return $map
}

function Get-ClaimCategories([string]$ClaimsLine) {
    # subject=CAT or subject=CAT@asserted|countersigned
    $out = @{}
    if (-not $ClaimsLine) { return $out }
    foreach ($part in ($ClaimsLine -split ';')) {
        $p = $part.Trim()
        if ($p -match '^([^=]+)=([A-D])(?:@(\w+))?$') {
            $out[$Matches[1]] = @{ category = $Matches[2]; verification = $(if ($Matches[3]) { $Matches[3] } else { $null }) }
        }
    }
    return $out
}

Write-Host "=== Mark detectors (read-only) ==="
Write-Host "Remote=$Remote Ref=$Ref Ledger=$LedgerPath"
Write-Host "HEAD=$(git rev-parse HEAD)"
Write-Host ""

Write-Host "--- Journey (local --all, last 20) ---"
git log --all --format="%h %ad %(trailers:key=Mark-Id,valueonly) %(trailers:key=Mark-Countersign,valueonly) %(trailers:key=Mark-Claims,valueonly)" --date=iso -n 20

Write-Host ""
Write-Host "--- Local HEAD BUILD trailers (five-field check) ---"
$headMap = Get-TrailerMap "HEAD"
$requiredBuild = @("Mark-Id", "Mark-Strand", "Mark-Role", "Mark-Claims", "Mark-Self-Certified")
# Find latest commit with Mark-Id if HEAD has none
$buildCommit = $null
if ($headMap.ContainsKey("Mark-Id") -and $headMap["Mark-Id"]) {
    $buildCommit = (git rev-parse HEAD)
} else {
    $buildCommit = git log --format="%H" --grep="Mark-Id:" -n 1 2>$null
    if (-not $buildCommit) {
        # trailers-based search
        $cands = git log --format="%H" -n 50
        foreach ($c in $cands) {
            $m = Get-TrailerMap $c
            if ($m.ContainsKey("Mark-Id") -and $m["Mark-Id"]) { $buildCommit = $c; break }
        }
    }
}
if ($buildCommit) {
    $bm = Get-TrailerMap $buildCommit
    Write-Host "build_commit=$buildCommit"
    foreach ($k in $requiredBuild) {
        $v = if ($bm.ContainsKey($k)) { $bm[$k] } else { "" }
        $ok = [bool]$v
        Write-Host ("  {0}: {1} ({2})" -f $k, $(if ($ok) { $v } else { "MISSING" }), $(if ($ok) { "ok" } else { "FAIL" }))
    }
    $fiveOk = ($requiredBuild | ForEach-Object { $bm.ContainsKey($_) -and $bm[$_] } | Where-Object { -not $_ }).Count -eq 0
    Write-Host "five_trailers=$(if ($fiveOk) { 'ok' } else { 'incomplete' })"
} else {
    Write-Host "No local Mark-Id commit found."
    $fiveOk = $false
}

Write-Host ""
Write-Host "--- D1 probe: Mark-Id on $Remote/$Ref without Mark-Countersign ---"
$remoteExists = $null
try { $remoteExists = git rev-parse --verify "$Remote/$Ref" 2>$null } catch { $remoteExists = $null }
$ids = @()
if (-not $remoteExists) {
    Write-Host "SKIP: $Remote/$Ref not available (fetch first). Detectors not live."
    $d1 = "skipped-no-remote"
} else {
    $withId = git log "$Remote/$Ref" --format="%(trailers:key=Mark-Id,valueonly)" | Where-Object { $_ -match '\S' }
    $withCs = git log "$Remote/$Ref" --format="%(trailers:key=Mark-Countersign,valueonly)" | Where-Object { $_ -match '\S' }
    $ids = @($withId | ForEach-Object { $_.Trim() } | Where-Object { $_ })
    $csCount = @($withCs).Count
    Write-Host "Mark-Id trailer lines on remote: $($ids.Count)"
    Write-Host "Mark-Countersign trailer lines on remote: $csCount"
    if ($ids.Count -eq 0) {
        Write-Host "RESULT: no marks on remote yet — D1 not live (spec only)."
        $d1 = "no-marks"
    } elseif ($csCount -eq 0) {
        Write-Host "FIRE D1 (coarse): marks present, zero countersigns on tip history."
        $d1 = "fire-coarse"
    } else {
        Write-Host "RESULT: countersigns present — refine with per-id join later."
        $d1 = "ok-or-needs-join"
    }
}

Write-Host ""
Write-Host "--- D2 probe: Mark-Claims vs Mark-Observed (local --all, category must not mutate) ---"
$d2 = "no-pair"
$csCommits = git log --all --format="%H" -n 80
$pairs = 0
$fires = 0
foreach ($c in $csCommits) {
    $m = Get-TrailerMap $c
    if (-not ($m.ContainsKey("Mark-Countersign") -and $m["Mark-Countersign"])) { continue }
    $mid = $m["Mark-Id"]
    $obs = $m["Mark-Observed"]
    if (-not $mid) { continue }
    # find BUILD claims for same id
    $claims = $null
    foreach ($b in $csCommits) {
        $bm = Get-TrailerMap $b
        if ($bm.ContainsKey("Mark-Id") -and $bm["Mark-Id"] -eq $mid -and $bm.ContainsKey("Mark-Claims") -and -not ($bm.ContainsKey("Mark-Countersign") -and $bm["Mark-Countersign"])) {
            $claims = $bm["Mark-Claims"]
            break
        }
    }
    if (-not $claims -or -not $obs) {
        Write-Host "  pair incomplete for $mid on $c"
        continue
    }
    $pairs++
    $cMap = Get-ClaimCategories $claims
    $oMap = Get-ClaimCategories $obs
    foreach ($subj in $cMap.Keys) {
        if ($oMap.ContainsKey($subj)) {
            if ($cMap[$subj].category -ne $oMap[$subj].category) {
                Write-Host "FIRE D2: $mid subject=$subj category $($cMap[$subj].category) -> $($oMap[$subj].category) (category must not mutate)"
                $fires++
            }
        }
    }
}
if ($pairs -eq 0) {
    Write-Host "RESULT: no countersign pairs yet — D2 not live."
    $d2 = "no-pair"
} elseif ($fires -gt 0) {
    $d2 = "fire-$fires"
} else {
    Write-Host "RESULT: $pairs pair(s), no category mutation."
    $d2 = "ok"
}

Write-Host ""
Write-Host "--- D4 probe: ledger ids vs remote trailers ---"
if (-not (Test-Path $LedgerPath)) {
    Write-Host "SKIP: ledger missing at $LedgerPath"
    $d4 = "no-ledger"
} else {
    $ledgerIds = Get-Content $LedgerPath | ForEach-Object {
        try { (($_ | ConvertFrom-Json).mark_id) } catch { $null }
    } | Where-Object { $_ }
    Write-Host "Ledger mark_ids: $($ledgerIds -join ', ')"
    if (-not $remoteExists) {
        $d4 = "skipped-no-remote"
    } elseif ($ids.Count -eq 0) {
        if ($ledgerIds.Count -gt 0) {
            Write-Host "NOTE: ledger has ids, remote has no Mark-Id trailers (expected until push)."
            $d4 = "ledger-only-pre-trailer"
        } else { $d4 = "empty" }
    } else {
        $remoteSet = [System.Collections.Generic.HashSet[string]]::new([string[]]$ids)
        $missing = @($ledgerIds | Where-Object { -not $remoteSet.Contains($_) })
        if ($missing.Count -gt 0) {
            Write-Host "FIRE D4 candidates: $($missing -join ', ')"
            $d4 = "fire-candidates"
        } else {
            $d4 = "ok"
        }
    }
}

Write-Host ""
Write-Host "--- D6 probe: Mark-Cert-Head vs countersign parent ---"
$d6 = "no-countersign"
$d6Fires = 0
$d6Seen = 0
foreach ($c in $csCommits) {
    $m = Get-TrailerMap $c
    if (-not ($m.ContainsKey("Mark-Countersign") -and $m["Mark-Countersign"])) { continue }
    $d6Seen++
    $certHead = if ($m.ContainsKey("Mark-Cert-Head")) { $m["Mark-Cert-Head"] } else { $null }
    $parent = (git rev-parse "$c^" 2>$null)
    if (-not $certHead) {
        Write-Host "FIRE D6: countersign $c missing Mark-Cert-Head"
        $d6Fires++
        continue
    }
    # full or abbreviated match
    $parentFull = git rev-parse $parent
    $certFull = git rev-parse --verify $certHead 2>$null
    if (-not $certFull) {
        Write-Host "FIRE D6: Mark-Cert-Head $certHead not resolvable (commit $c)"
        $d6Fires++
    } elseif ($parentFull -ne $certFull) {
        Write-Host "FIRE D6: countersign $c parent=$parentFull cert_head=$certFull"
        $d6Fires++
    } else {
        Write-Host "OK D6: $c parent matches Mark-Cert-Head"
    }
}
if ($d6Seen -eq 0) {
    Write-Host "RESULT: no countersign commits — D6 not live."
    $d6 = "no-countersign"
} elseif ($d6Fires -gt 0) {
    $d6 = "fire-$d6Fires"
} else {
    $d6 = "ok"
}

function Test-ArtifactDrift([string]$BuildHead, [string]$SurveyHead, [string[]]$Artifacts) {
    # Returns list of drifted paths. Skips ledger bookkeeping.
    $drifted = [System.Collections.Generic.List[string]]::new()
    $buildFull = git rev-parse --verify $BuildHead 2>$null
    $surveyFull = git rev-parse --verify $SurveyHead 2>$null
    if (-not $buildFull -or -not $surveyFull) {
        return ,@("UNRESOLVABLE:$BuildHead..$SurveyHead")
    }
    if ($buildFull -eq $surveyFull) {
        return ,@()
    }
    foreach ($raw in $Artifacts) {
        $a = [string]$raw
        if (-not $a) { continue }
        # Exclude ledger — stamp commits must not false-positive D7
        if ($a -match '(^|/)MARKS\.jsonl$') { continue }
        $pathspecs = @($a)
        if ($a -match '[\\/]$' -or $a -eq 'ops/marks' -or $a -eq 'ops/marks/') {
            $pathspecs = @($a.TrimEnd('/\'), ':!ops/marks/MARKS.jsonl')
        } elseif ($a -eq 'ops/marks' -or $a.TrimEnd('/\') -eq 'ops/marks') {
            $pathspecs = @('ops/marks', ':!ops/marks/MARKS.jsonl')
        }
        # git diff --quiet: exit 1 = differences, 0 = same, 2 = error
        $null = git diff --quiet $buildFull $surveyFull -- @pathspecs 2>$null
        $code = $LASTEXITCODE
        if ($code -eq 1) {
            [void]$drifted.Add($a)
        } elseif ($code -gt 1) {
            [void]$drifted.Add("$a(diff-error)")
        }
    }
    return ,$drifted.ToArray()
}

Write-Host ""
Write-Host "--- D7 probe: claim drift (ledger.head_sha vs survey head on claimed artifacts) ---"
Write-Host "BUILD head_sha = tree claims describe; VERIFY CertHead = tree surveyed. D6 does not bind them."
$d7 = "no-marks"
$d7Fires = 0
$d7Checked = 0
$surveyDefault = git rev-parse --verify $CertHead 2>$null
if (-not $surveyDefault) {
    Write-Host "SKIP D7: -CertHead $CertHead not resolvable"
    $d7 = "bad-cert-head"
} elseif (-not (Test-Path $LedgerPath)) {
    $d7 = "no-ledger"
} else {
    $rows = Get-Content $LedgerPath | ForEach-Object {
        try { $_ | ConvertFrom-Json } catch { $null }
    } | Where-Object { $_ -and $_.mark_id }

    foreach ($row in $rows) {
        $mid = [string]$row.mark_id
        $buildHead = [string]$row.head_sha
        if (-not $buildHead) { $buildHead = [string]$row.commit }
        if (-not $buildHead) {
            Write-Host "  ${mid}: no ledger head_sha — skip"
            continue
        }
        # Prefer Mark-Cert-Head from countersign for this mark; else -CertHead (preflight)
        $survey = $surveyDefault
        $foundCs = $false
        foreach ($c in $csCommits) {
            $m = Get-TrailerMap $c
            if ($m.ContainsKey("Mark-Id") -and $m["Mark-Id"] -eq $mid -and $m.ContainsKey("Mark-Countersign") -and $m["Mark-Countersign"]) {
                if ($m.ContainsKey("Mark-Cert-Head") -and $m["Mark-Cert-Head"]) {
                    $survey = $m["Mark-Cert-Head"]
                    $foundCs = $true
                }
                break
            }
        }
        $arts = @()
        if ($row.artifacts) { $arts = @($row.artifacts | ForEach-Object { [string]$_ }) }
        if ($arts.Count -eq 0) {
            Write-Host "  ${mid}: no artifacts list — skip"
            continue
        }
        $d7Checked++
        $mode = if ($foundCs) { "countersign-Cert-Head" } else { "preflight-CertHead=$CertHead" }
        $drift = Test-ArtifactDrift $buildHead $survey $arts
        $bhShort = $buildHead.Substring(0, [Math]::Min(8, $buildHead.Length))
        if ($drift.Count -eq 0) {
            Write-Host "OK D7: ${mid} ($mode) build=${bhShort} survey — no claimed-path drift"
        } else {
            Write-Host "FIRE D7: ${mid} ($mode) drifted: $($drift -join ', ')"
            Write-Host "  REFUSE countersign. BUILD must mint a new Mark-Id (VERIFY never mints)."
            $d7Fires++
        }
    }
    if ($d7Checked -eq 0) {
        $d7 = "no-checkable-rows"
    } elseif ($d7Fires -gt 0) {
        $d7 = "fire-$d7Fires"
    } else {
        $d7 = "ok"
    }
}

Write-Host ""
Write-Host "--- Divergence note (ahead/behind origin) ---"
git status -sb | Select-Object -First 1

Write-Host ""
Write-Host "=== Summary ==="
Write-Host "five_trailers_local=$(if ($fiveOk) { 'ok' } else { 'incomplete' }) d1=$d1 d2=$d2 d4=$d4 d6=$d6 d7=$d7"
Write-Host "Honest status: detectors remain Category B until countersigned mark with Mark-Cert-Head exists on origin."
Write-Host "category is never promoted by verification; B stays B; verified-C is real; D7 binds BUILD claim tree to VERIFY survey."
