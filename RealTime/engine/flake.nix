{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/5083ec887760adfe12af64830a66807423a859a7?narHash=sha256-D1FNZ70NmQEwNxpSSdTXCSklBH1z2isPR84J6DQrJGs%3D";
  };

  outputs = { nixpkgs, ... }:
    {
      devShells.x86_64-linux.default =
        let
          pkgs = nixpkgs.legacyPackages.x86_64-linux;
        in
        pkgs.mkShell {
          buildInputs = with pkgs; [ cmake glfw glm assimp cimg fontconfig xorg.libX11 apitrace ccls ];
        };
    };
}
