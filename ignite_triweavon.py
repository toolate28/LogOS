import os
import sys
import time
import subprocess
import threading
import http.server
import socketserver

def clear_screen():
    os.system('cls' if os.name == 'nt' else 'clear')

def print_argonath_splash():
    splash = """
    =============================================================================
         █████╗ ██████╗  ██████╗  ██████╗ ███╗   ██╗ █████╗ ████████╗██╗  ██╗
        ██╔══██╗██╔══██╗██╔════╝ ██╔═══██╗████╗  ██║██╔══██╗╚══██╔══╝██║  ██║
        ███████║██████╔╝██║  ███╗██║   ██║██╔██╗ ██║███████║   ██║   ███████║
        ██╔══██║██╔══██╗██║   ██║██║   ██║██║╚██╗██║██╔══██║   ██║   ██╔══██║
        ██║  ██║██║  ██║╚██████╔╝╚██████╔╝██║ ╚████║██║  ██║   ██║   ██║  ██║
        ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝
    =============================================================================
                          S O V E R E I G N   R E L E A S E
         "Thus far, and no farther, without the consent of the sovereign will."
    =============================================================================
    """
    print(splash)
    time.sleep(1)

def run_step(title, command, cwd):
    print(f"\n[*] INITIATING PROTOCOL: {title}")
    print("[*] Generating Verification Docket...")
    time.sleep(0.5)
    
    try:
        process = subprocess.Popen(
            command,
            cwd=cwd,
            shell=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True
        )
        
        output = ""
        for line in process.stdout:
            print(f"    | {line.strip()}")
            output += line
            
        process.wait()
        
        if process.returncode == 0:
            print(f"\n[✔] PROTOCOL VERIFIED: {title}. Atom-Tag Docket Signed.")
            return True, output
        else:
            print(f"\n[!] PROTOCOL FAILED: {title}. Argonath Seal Withheld.")
            return False, output
            
    except Exception as e:
        print(f"[!] SYSTEM ERROR during {title}: {str(e)}")
        return False, str(e)

def start_site_server():
    print("[*] Igniting Dual-Facing TDA Coherence Site...")
    try:
        os.chdir(r"c:\Users\Matthew Ruhnau\reson8\crates\coherence-mcp\coherence-site\public")
        PORT = 8080
        Handler = http.server.SimpleHTTPRequestHandler
        with socketserver.TCPServer(("", PORT), Handler) as httpd:
            print(f"[✔] TDA Site Active at http://localhost:{PORT}/gate/Gate.html")
            httpd.serve_forever()
    except Exception as e:
        # Fallback if path wrong
        os.chdir(r"c:\Users\Matthew Ruhnau\reson8\coherence-mcp\coherence-site\public")
        PORT = 8080
        Handler = http.server.SimpleHTTPRequestHandler
        with socketserver.TCPServer(("", PORT), Handler) as httpd:
            print(f"[✔] TDA Site Active at http://localhost:{PORT}/gate/Gate.html")
            httpd.serve_forever()

def start_gear_core():
    print("[*] Igniting GEAR Core Telemetry (ws://127.0.0.1:8089)...")
    try:
        subprocess.Popen(["python", "gear_core.py"], cwd=r"c:\Users\Matthew Ruhnau\reson8")
        print("[✔] WebSocket Telemetry Bound. Resonance Active.")
    except Exception as e:
        print("[!] Failed to start GEAR Core.")

def main():
    clear_screen()
    print_argonath_splash()
    
    mcp_dir = r"c:\Users\Matthew Ruhnau\reson8\crates\coherence-mcp"
    if not os.path.exists(mcp_dir):
        mcp_dir = r"c:\Users\Matthew Ruhnau\coherence-mcp"
        
    print("[*] TARGETING ARCHITECTURE ROOT:", mcp_dir)
    print("-" * 77)
    
    # 1. Verification Docket: Type Compilation
    success, _ = run_step("TypeScript Compilation Phase", "npm run build", cwd=mcp_dir)
    if not success: sys.exit(1)
    
    # 2. Verification Docket: 570 Tests Boundary Condition
    success, _ = run_step("Mathematical Test Matrix Validation", "npm test", cwd=mcp_dir)
    if not success: sys.exit(1)
        
    print("-" * 77)
    print("[*] ALL PROTOCOLS FORMALIZED. DUAL-FACING ARTIFACTS VALIDATED.")
    print("[*] The Argonath grants the seal. Transitioning to Deployment Mode.\n")
    
    # Bring up the architecture
    threading.Thread(target=start_gear_core, daemon=True).start()
    threading.Thread(target=start_site_server, daemon=True).start()
    
    time.sleep(2)
    print("\n=============================================================================")
    print(" [OS]: TRI-WEAVON ARCHITECTURE ONLINE")
    print(" [UI]: Open http://localhost:8080/gate/Gate.html to access the Argonath.")
    print(" [UI]: Open http://localhost:8080/flow/index.html for Live TDA.")
    print("=============================================================================")
    
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("\n[*] Shutting down TriWeavon Architecture.")

if __name__ == "__main__":
    main()
