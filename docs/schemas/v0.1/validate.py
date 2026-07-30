import json, os, sys
from jsonschema import Draft202012Validator

def load(p): return json.load(open(p))
def check(schema_path, doc_path, expect_pass=True):
    v = Draft202012Validator(load(schema_path))
    errs = sorted(v.iter_errors(load(doc_path)), key=lambda e: list(e.path))
    tag = "PASS" if not errs else "FAIL"
    print(f"[{tag}] {doc_path} vs {schema_path} ({len(errs)} errors)")
    for e in errs[:14]:
        loc = "/".join(map(str,e.path)) or "<root>"
        print(f"   - {loc}: {e.message[:110]}")
    ok = (not errs) == expect_pass
    return ok

r = True
# schemas are themselves valid 2020-12
for s in ["certificate.schema.json","handoff_packet.schema.json","ledger_entry.schema.json","claims_register.schema.json"]:
    Draft202012Validator.check_schema(load(s)); print(f"[OK ] schema well-formed: {s}")
r &= check("handoff_packet.schema.json","vectors/packet_example_sa01.json",True)
r &= check("certificate.schema.json","vectors/certificate_example_lane_d.json",True)
print()
print("=== pre-freeze cert vs v0.1 schema (EXPECT FAIL -> computed migration delta) ===")
# Prefer Claude Projects mount when present; else filed local vector
_fail = "/mnt/project/existence_certificate_mog.json"
if not os.path.isfile(_fail):
    _fail = "vectors/existence_certificate_pre_freeze.json"
r &= check("certificate.schema.json", _fail, False)
print()
print("ALL EXPECTATIONS MET" if r else "EXPECTATION MISMATCH"); sys.exit(0 if r else 1)
