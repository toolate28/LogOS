from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import subprocess
import json
from pathlib import Path
from datetime import datetime

app = FastAPI(title="TERAFAB - Crystalline World Builder")

BASE = Path(__import__("os").environ.get("LOGOS_9P", r"F:\Users\Matthew Ruhnau\LogOS\9P2000.L"))
BLUEPRINTS_DIR = BASE / "crates" / "blueprints"


class DeployRequest(BaseModel):
    blueprint: str
    world: str = "WORLD_MUSEUM_CORE"
    position: str = "~ -200 64 0"
    paste_mode: str = "replace"


@app.post("/deploy")
async def deploy_blueprint(req: DeployRequest):
    blueprint_path = BLUEPRINTS_DIR / f"{req.blueprint}.yml"

    if not blueprint_path.exists():
        raise HTTPException(404, "Blueprint not found")

    try:
        command = f"/fawe paste {req.blueprint} {req.position} -w {req.world}"
        result = subprocess.run(
            ["mcrcon", "-H", "127.0.0.1", "-p", "YOUR_RCON_PASSWORD", command],
            capture_output=True, text=True, timeout=30
        )

        return {
            "status": "deployed",
            "blueprint": req.blueprint,
            "position": req.position,
            "timestamp": datetime.utcnow().isoformat(),
            "fawe_output": result.stdout.strip()
        }
    except Exception as e:
        raise HTTPException(500, f"Deployment failed: {str(e)}")


@app.get("/blueprints")
async def list_blueprints():
    blueprints = [f.stem for f in BLUEPRINTS_DIR.glob("*.yml")]
    return {"blueprints": blueprints, "count": len(blueprints)}
