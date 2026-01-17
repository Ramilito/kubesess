class Kubesess < Formula
  desc "Manage multiple kubernetes cluster at the same time"
  homepage "https://github.com/Ramilito/kubesess"
  url "https://github.com/Ramilito/kubesess/archive/refs/tags/3.0.0.tar.gz"
  sha256 "PLACEHOLDER"
  license "MIT"
  head "https://github.com/Ramilito/kubesess.git", branch: "main"

  # bottle block will be added by Homebrew CI

  depends_on "rust" => :build
  depends_on "kubernetes-cli" => :test

  def install
    system "cargo", "install", *std_cargo_args
  end

  def caveats
    <<~EOS
      To activate kubesess shell integration, add to your shell config:

      For bash (~/.bashrc):
        eval "$(kubesess init bash)"

      For zsh (~/.zshrc):
        eval "$(kubesess init zsh)"

      For fish (~/.config/fish/config.fish):
        kubesess init fish | source

      For PowerShell:
        Invoke-Expression (&kubesess init powershell)

      This provides the commands: kc, kcd, kn, knd
    EOS
  end

  test do
    (testpath/".kube/config").write <<~YAML
      kind: Config
      apiVersion: v1
      current-context: docker-desktop
      preferences: {}
      clusters:
      - cluster:
          server: https://kubernetes.docker.internal:6443
        name: docker-desktop
      contexts:
      - context:
          namespace: monitoring
          cluster: docker-desktop
          user: docker-desktop
        name: docker-desktop
      users:
      - user:
        name: docker-desktop
    YAML

    output = shell_output("#{bin}/kubesess context -v docker-desktop 2>&1")
    assert_match "docker-desktop", output
  end
end
