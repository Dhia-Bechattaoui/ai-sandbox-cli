class AiSandboxCli < Formula
  desc "A high-performance CLI utility for secure execution of untrusted AI snippets"
  homepage "https://github.com/Dhia-Bechattaoui/ai-sandbox-cli"
  url "https://github.com/Dhia-Bechattaoui/ai-sandbox-cli/archive/refs/tags/v0.0.3.tar.gz"
  sha256 "REPLACE_WITH_SHA256_HASH_OF_RELEASE_TARBALL"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "*std_cargo_args"
  end

  test do
    system "#{bin}/ai-sandbox-cli", "--help"
  end
end
