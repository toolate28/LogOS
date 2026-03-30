{
  description = "G.E.A.R. Pipeline Environment for GLF OS 25.11 Phoenix Pulsar";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    pythonEnv = pkgs.python311.withPackages (ps: with ps; [
      # Core ML / Quantum
      qiskit
      numpy
      scipy
      pandas
      
      # Web / Async
      websockets
      asyncio
      
      # DSPy and Ax may require custom derivations if missing in nixpkgs, 
      # but specified here assuming an accessible overlay or nixpkgs bump.
      dspy-ai
    ]);
  in {
    nixosConfigurations.phoenix-pulsar = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ({ config, lib, pkgs, ... }: {
          systemd.services.gear-core = {
            description = "G.E.A.R. Topological Python Runtime";
            after = [ "network.target" ];
            wantedBy = [ "multi-user.target" ];
            serviceConfig = {
              ExecStart = "${pythonEnv}/bin/python /opt/gear/gear_core.py";
              Restart = "always";
              EnvironmentFile = "/etc/coherence/coherence.env";
            };
          };
        })
      ];
    };

    devShells.${system}.default = pkgs.mkShell {
      buildInputs = [ 
        pythonEnv 
        pkgs.cloudflared 
      ];
      
      shellHook = ''
        export PYTHONPATH="$PWD:$PYTHONPATH"
        echo "G.E.A.R. Environment Activated."
        echo "Constraints: α + ω = 15 [6V+9P Levin-Wen lattice fixed]"
        echo "Hardware target: Anduril 4.e-ATX Open Frame (AMD 5600H + RTX 5090)"
      '';
    };
  };
}
