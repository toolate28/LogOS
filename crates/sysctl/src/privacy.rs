//! Privacy and telemetry controls — OOShutup-class reach.
//!
//! Maps Windows telemetry, tracking, and data collection settings
//! to toggleable registry/group-policy entries. Each tweak is a
//! [`PrivacyTweak`] with a category, risk level, and reversibility flag.

use serde::{Deserialize, Serialize};

/// Risk level of applying a privacy tweak.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TweakRisk {
    /// Safe — no functional impact (e.g. disable ad ID)
    Safe,
    /// Moderate — may disable some convenience features (e.g. Cortana)
    Moderate,
    /// Aggressive — may break Microsoft services (e.g. disable Connected User Experiences)
    Aggressive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TweakCategory {
    Telemetry,
    Advertising,
    Cortana,
    LocationTracking,
    DiagnosticData,
    ActivityHistory,
    AppPermissions,
    WindowsUpdate,
    CloudContent,
    EdgeTelemetry,
    OfficeTelemetry,
    GameBar,
    WidgetsNews,
    SearchHighlights,
    CopilotRecall,
    StartMenuAds,
    LockScreenAds,
    SuggestedApps,
    BackgroundApps,
    Defender,
    Firewall,
    Services,
}

/// A single privacy/telemetry tweak.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyTweak {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: TweakCategory,
    pub risk: TweakRisk,
    pub reversible: bool,
    /// Registry path + value name + desired value
    pub registry: Option<RegistryTweak>,
    /// PowerShell command to apply
    pub apply_cmd: String,
    /// PowerShell command to revert
    pub revert_cmd: String,
    /// Current state (true = privacy-preserving, false = default/tracking)
    pub applied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryTweak {
    pub path: String,
    pub value_name: String,
    pub value_type: String,
    pub value_data: String,
    pub default_data: String,
}

/// Built-in privacy tweaks — the "OOShutup reach".
pub fn default_tweaks() -> Vec<PrivacyTweak> {
    vec![
        // ── Telemetry ────────────────────────────────────────────────────
        PrivacyTweak {
            id: "telemetry_disable".into(),
            name: "Disable Telemetry".into(),
            description: "Set diagnostic data to Security (minimal)".into(),
            category: TweakCategory::Telemetry,
            risk: TweakRisk::Safe,
            reversible: true,
            registry: Some(RegistryTweak {
                path: r"HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection".into(),
                value_name: "AllowTelemetry".into(),
                value_type: "DWORD".into(),
                value_data: "0".into(),
                default_data: "3".into(),
            }),
            apply_cmd: r#"Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection" -Name "AllowTelemetry" -Value 0 -Type DWord -Force"#.into(),
            revert_cmd: r#"Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection" -Name "AllowTelemetry" -Force -ErrorAction SilentlyContinue"#.into(),
            applied: false,
        },

        // ── Advertising ID ───────────────────────────────────────────────
        PrivacyTweak {
            id: "ad_id_disable".into(),
            name: "Disable Advertising ID".into(),
            description: "Prevent apps from using advertising ID for cross-app tracking".into(),
            category: TweakCategory::Advertising,
            risk: TweakRisk::Safe,
            reversible: true,
            registry: Some(RegistryTweak {
                path: r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo".into(),
                value_name: "Enabled".into(),
                value_type: "DWORD".into(),
                value_data: "0".into(),
                default_data: "1".into(),
            }),
            apply_cmd: r#"Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo" -Name "Enabled" -Value 0 -Type DWord -Force"#.into(),
            revert_cmd: r#"Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo" -Name "Enabled" -Value 1 -Type DWord -Force"#.into(),
            applied: false,
        },

        // ── Activity History ─────────────────────────────────────────────
        PrivacyTweak {
            id: "activity_history_disable".into(),
            name: "Disable Activity History".into(),
            description: "Stop Windows from collecting activity history".into(),
            category: TweakCategory::ActivityHistory,
            risk: TweakRisk::Safe,
            reversible: true,
            registry: Some(RegistryTweak {
                path: r"HKLM\SOFTWARE\Policies\Microsoft\Windows\System".into(),
                value_name: "EnableActivityFeed".into(),
                value_type: "DWORD".into(),
                value_data: "0".into(),
                default_data: "1".into(),
            }),
            apply_cmd: r#"Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\System" -Name "EnableActivityFeed" -Value 0 -Type DWord -Force; Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\System" -Name "PublishUserActivities" -Value 0 -Type DWord -Force; Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\System" -Name "UploadUserActivities" -Value 0 -Type DWord -Force"#.into(),
            revert_cmd: r#"Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\System" -Name "EnableActivityFeed" -Force -ErrorAction SilentlyContinue; Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\System" -Name "PublishUserActivities" -Force -ErrorAction SilentlyContinue"#.into(),
            applied: false,
        },

        // ── Cortana ──────────────────────────────────────────────────────
        PrivacyTweak {
            id: "cortana_disable".into(),
            name: "Disable Cortana".into(),
            description: "Disable Cortana assistant and web search in Start".into(),
            category: TweakCategory::Cortana,
            risk: TweakRisk::Moderate,
            reversible: true,
            registry: Some(RegistryTweak {
                path: r"HKLM\SOFTWARE\Policies\Microsoft\Windows\Windows Search".into(),
                value_name: "AllowCortana".into(),
                value_type: "DWORD".into(),
                value_data: "0".into(),
                default_data: "1".into(),
            }),
            apply_cmd: r#"New-Item -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search" -Force | Out-Null; Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search" -Name "AllowCortana" -Value 0 -Type DWord -Force; Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search" -Name "DisableWebSearch" -Value 1 -Type DWord -Force"#.into(),
            revert_cmd: r#"Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\Windows Search" -Name "AllowCortana" -Force -ErrorAction SilentlyContinue"#.into(),
            applied: false,
        },

        // ── Location Tracking ────────────────────────────────────────────
        PrivacyTweak {
            id: "location_disable".into(),
            name: "Disable Location Tracking".into(),
            description: "Turn off Windows location services".into(),
            category: TweakCategory::LocationTracking,
            risk: TweakRisk::Moderate,
            reversible: true,
            registry: Some(RegistryTweak {
                path: r"HKLM\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors".into(),
                value_name: "DisableLocation".into(),
                value_type: "DWORD".into(),
                value_data: "1".into(),
                default_data: "0".into(),
            }),
            apply_cmd: r#"New-Item -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors" -Force | Out-Null; Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors" -Name "DisableLocation" -Value 1 -Type DWord -Force"#.into(),
            revert_cmd: r#"Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\LocationAndSensors" -Name "DisableLocation" -Force -ErrorAction SilentlyContinue"#.into(),
            applied: false,
        },

        // ── Copilot / Recall ─────────────────────────────────────────────
        PrivacyTweak {
            id: "copilot_recall_disable".into(),
            name: "Disable Copilot + Recall".into(),
            description: "Disable Windows Copilot sidebar and Recall screenshot feature".into(),
            category: TweakCategory::CopilotRecall,
            risk: TweakRisk::Safe,
            reversible: true,
            registry: Some(RegistryTweak {
                path: r"HKCU\Software\Policies\Microsoft\Windows\WindowsCopilot".into(),
                value_name: "TurnOffWindowsCopilot".into(),
                value_type: "DWORD".into(),
                value_data: "1".into(),
                default_data: "0".into(),
            }),
            apply_cmd: r#"New-Item -Path "HKCU:\Software\Policies\Microsoft\Windows\WindowsCopilot" -Force | Out-Null; Set-ItemProperty -Path "HKCU:\Software\Policies\Microsoft\Windows\WindowsCopilot" -Name "TurnOffWindowsCopilot" -Value 1 -Type DWord -Force; New-Item -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsAI" -Force | Out-Null; Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsAI" -Name "DisableAIDataAnalysis" -Value 1 -Type DWord -Force"#.into(),
            revert_cmd: r#"Remove-ItemProperty -Path "HKCU:\Software\Policies\Microsoft\Windows\WindowsCopilot" -Name "TurnOffWindowsCopilot" -Force -ErrorAction SilentlyContinue; Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsAI" -Name "DisableAIDataAnalysis" -Force -ErrorAction SilentlyContinue"#.into(),
            applied: false,
        },

        // ── Start Menu Ads / Suggestions ─────────────────────────────────
        PrivacyTweak {
            id: "start_ads_disable".into(),
            name: "Disable Start Menu Ads & Suggestions".into(),
            description: "Remove suggested apps and promotional content from Start".into(),
            category: TweakCategory::StartMenuAds,
            risk: TweakRisk::Safe,
            reversible: true,
            registry: Some(RegistryTweak {
                path: r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager".into(),
                value_name: "SubscribedContent-338389Enabled".into(),
                value_type: "DWORD".into(),
                value_data: "0".into(),
                default_data: "1".into(),
            }),
            apply_cmd: r#"$cdm = "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager"; @("SubscribedContent-338389Enabled","SubscribedContent-310093Enabled","SubscribedContent-338388Enabled","SubscribedContent-338393Enabled","SilentInstalledAppsEnabled","SoftLandingEnabled","SystemPaneSuggestionsEnabled") | ForEach-Object { Set-ItemProperty -Path $cdm -Name $_ -Value 0 -Type DWord -Force }"#.into(),
            revert_cmd: r#"$cdm = "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager"; @("SubscribedContent-338389Enabled","SubscribedContent-310093Enabled","SubscribedContent-338388Enabled","SubscribedContent-338393Enabled","SilentInstalledAppsEnabled","SoftLandingEnabled","SystemPaneSuggestionsEnabled") | ForEach-Object { Set-ItemProperty -Path $cdm -Name $_ -Value 1 -Type DWord -Force }"#.into(),
            applied: false,
        },

        // ── Background Apps ──────────────────────────────────────────────
        PrivacyTweak {
            id: "background_apps_disable".into(),
            name: "Disable Background Apps".into(),
            description: "Prevent UWP apps from running in the background".into(),
            category: TweakCategory::BackgroundApps,
            risk: TweakRisk::Moderate,
            reversible: true,
            registry: Some(RegistryTweak {
                path: r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications".into(),
                value_name: "GlobalUserDisabled".into(),
                value_type: "DWORD".into(),
                value_data: "1".into(),
                default_data: "0".into(),
            }),
            apply_cmd: r#"Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications" -Name "GlobalUserDisabled" -Value 1 -Type DWord -Force"#.into(),
            revert_cmd: r#"Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\BackgroundAccessApplications" -Name "GlobalUserDisabled" -Value 0 -Type DWord -Force"#.into(),
            applied: false,
        },

        // ── Widgets / News ───────────────────────────────────────────────
        PrivacyTweak {
            id: "widgets_disable".into(),
            name: "Disable Widgets & News Feed".into(),
            description: "Remove taskbar widgets panel and news feed".into(),
            category: TweakCategory::WidgetsNews,
            risk: TweakRisk::Safe,
            reversible: true,
            registry: Some(RegistryTweak {
                path: r"HKLM\SOFTWARE\Policies\Microsoft\Dsh".into(),
                value_name: "AllowNewsAndInterests".into(),
                value_type: "DWORD".into(),
                value_data: "0".into(),
                default_data: "1".into(),
            }),
            apply_cmd: r#"New-Item -Path "HKLM:\SOFTWARE\Policies\Microsoft\Dsh" -Force | Out-Null; Set-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Dsh" -Name "AllowNewsAndInterests" -Value 0 -Type DWord -Force"#.into(),
            revert_cmd: r#"Remove-ItemProperty -Path "HKLM:\SOFTWARE\Policies\Microsoft\Dsh" -Name "AllowNewsAndInterests" -Force -ErrorAction SilentlyContinue"#.into(),
            applied: false,
        },

        // ── Game Bar / DVR ───────────────────────────────────────────────
        PrivacyTweak {
            id: "gamebar_disable".into(),
            name: "Disable Game Bar & DVR".into(),
            description: "Turn off Xbox Game Bar overlay and background recording".into(),
            category: TweakCategory::GameBar,
            risk: TweakRisk::Safe,
            reversible: true,
            registry: Some(RegistryTweak {
                path: r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR".into(),
                value_name: "AppCaptureEnabled".into(),
                value_type: "DWORD".into(),
                value_data: "0".into(),
                default_data: "1".into(),
            }),
            apply_cmd: r#"Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR" -Name "AppCaptureEnabled" -Value 0 -Type DWord -Force; Set-ItemProperty -Path "HKCU:\System\GameConfigStore" -Name "GameDVR_Enabled" -Value 0 -Type DWord -Force"#.into(),
            revert_cmd: r#"Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR" -Name "AppCaptureEnabled" -Value 1 -Type DWord -Force; Set-ItemProperty -Path "HKCU:\System\GameConfigStore" -Name "GameDVR_Enabled" -Value 1 -Type DWord -Force"#.into(),
            applied: false,
        },

        // ── Search Highlights ────────────────────────────────────────────
        PrivacyTweak {
            id: "search_highlights_disable".into(),
            name: "Disable Search Highlights".into(),
            description: "Remove trending/promotional content from Windows Search".into(),
            category: TweakCategory::SearchHighlights,
            risk: TweakRisk::Safe,
            reversible: true,
            registry: Some(RegistryTweak {
                path: r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\SearchSettings".into(),
                value_name: "IsDynamicSearchBoxEnabled".into(),
                value_type: "DWORD".into(),
                value_data: "0".into(),
                default_data: "1".into(),
            }),
            apply_cmd: r#"Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\SearchSettings" -Name "IsDynamicSearchBoxEnabled" -Value 0 -Type DWord -Force"#.into(),
            revert_cmd: r#"Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\SearchSettings" -Name "IsDynamicSearchBoxEnabled" -Value 1 -Type DWord -Force"#.into(),
            applied: false,
        },

        // ── Connected User Experiences ───────────────────────────────────
        PrivacyTweak {
            id: "connected_user_exp_disable".into(),
            name: "Disable Connected User Experiences".into(),
            description: "Block DiagTrack and dmwappushservice telemetry services".into(),
            category: TweakCategory::Telemetry,
            risk: TweakRisk::Aggressive,
            reversible: true,
            registry: None,
            apply_cmd: r#"Stop-Service -Name "DiagTrack" -Force -ErrorAction SilentlyContinue; Set-Service -Name "DiagTrack" -StartupType Disabled; Stop-Service -Name "dmwappushservice" -Force -ErrorAction SilentlyContinue; Set-Service -Name "dmwappushservice" -StartupType Disabled"#.into(),
            revert_cmd: r#"Set-Service -Name "DiagTrack" -StartupType Automatic; Start-Service -Name "DiagTrack"; Set-Service -Name "dmwappushservice" -StartupType Automatic; Start-Service -Name "dmwappushservice""#.into(),
            applied: false,
        },
    ]
}
