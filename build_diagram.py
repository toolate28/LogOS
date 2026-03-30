import sys
import json
import subprocess

def main():
    # Run cargo metadata
    result = subprocess.run(["cargo", "metadata", "--format-version", "1", "--no-deps"], capture_output=True, text=True, env={"RUSTC_WRAPPER": "", **sys.modules['os'].environ})
    if result.returncode != 0:
        print("Failed to run cargo metadata:")
        print(result.stderr)
        sys.exit(1)
    
    metadata = json.loads(result.stdout)
    packages = metadata.get("packages", [])
    
    workspace_members = metadata.get("workspace_members", [])
    
    # Filter workspace packages
    ws_packages = [pkg for pkg in packages if pkg["id"] in workspace_members]
    ws_package_names = set(pkg["name"] for pkg in ws_packages)
    
    # Map dependencies
    diagram_lines = ["```mermaid", "graph TD"]
    diagram_lines.append("  subgraph Reson8_Workspace [reson8 Cargo Workspace]")
    
    for pkg in ws_packages:
        name = pkg["name"]
        clean_name = name.replace("-", "_")
        diagram_lines.append(f"    {clean_name}[\"{name}\"]")
        for dep in pkg.get("dependencies", []):
            if dep["name"] in ws_package_names:
                dep_name = dep["name"].replace("-", "_")
                diagram_lines.append(f"    {clean_name} --> {dep_name}")
                
    diagram_lines.append("  end")
    diagram_lines.append("```")
    
    print("\n".join(diagram_lines))

if __name__ == "__main__":
    main()
