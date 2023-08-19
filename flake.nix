{
  description = "A very basic flake";

  outputs = { self, nixpkgs }: let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in {
    packages.x86_64-linux.hello = nixpkgs.legacyPackages.x86_64-linux.hello;
    packages.x86_64-linux.default = self.packages.x86_64-linux.hello;

    devShells.x86_64-linux.default = pkgs.mkShell {
      #nativeBuildInputs = with pkgs; [
      #  clang
      #  clang-tools
      #];
      buildInputs = with pkgs; [
        gnumake
        gdb

        clang-tools
        clang

        man-pages
        man-pages-posix

        graphviz

        nodejs
        yarn
      ];
    };
  };
}
