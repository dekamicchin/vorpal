<h1>Vorpal</h1>
<h2>The razor sharp AI model CLI downloader</h2>
<p>Vorpal is a small, rust-based, command-line utility to download Stable Diffusion models and LoRAs from Civitai (and potentially Huggingface, in the future).  Searching and downloading the latest Stable Diffusion models and LoRAs can be painless and lightning-fast.</p>
<br>
<h2>Features</h2>
<p>With Vorpal, you can:</p>
<ul>
  <li>Query Civitai for Stable Diffusion models and LoRAs</li>
  <li>Download the latest version of a model by name or keyword</li>
  <li>Download the model's metadata (such as activation/trigger words)</li>
  <li>Have models automatically be put in a set directory (via environment variable), or...</li>
  <li>Use the CLI to set the download directory for a specific download</li>
  <li>All of the above, in that order, in mere seconds</li>
  <li>Moves at the speed of Rust (that is to say, extremely fast)</li>
</ul>
<h2>Installation</h2>
<br>
<h3>Linux, MacOS, and Windows (via WSL)</h3>
<p>Ensure you have the latest version of the [Rust](https://www.rust-lang.org/tools/install) language</p>
<p>Use this command to install Rust on Linux, MacOS, or Unix-likes (BSD)</p>

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

<p>(Recommended) navigate to a directory of your choice. I prefer a 'builds' directory in my home</p>
      mkdir $HOME/builds && cd $HOME/builds

<p>Clone the vorpal repo</p>
      git clone https://github.com/dekamicchin/vorpal.git
<p>Install Vorpal</p>
      cargo install

<p>Add cargo executables to your PATH so you can use it like you would a normal command-line program</p>
      echo 'export PATH=$HOME/.cargo/bin:$PATH' >> $HOME/.bash_profile
<p>MacOS users or zsh enjoyers: add the PATH adjustment to your corresponding shell rc</p>
      echo 'export PATH=$HOME/.cargo/bin:$PATH' >> $HOME/.zshrc
<p>Homebrew, Arch, and Debian packages are incoming, just please give me time.</p>
<br>
<p>Vorpal is lightweight, fast, and programmable. It is suitable for a multitude of environments, such as desktops or servers. You can easily find and download whatever model you need through easy to use and familiar command-line controls. Vorpal is also suitable for servers and enterprises, and allows machines to download models themselves without the need for rsync, ssh file transfers, or bloated git repos.</p>
<p>I want to focus on the downloading aspect, and make it as easy as possible to find and download models of more types and sources.</p>
<p>I built this because the current way of downloading models is annoying. The Civitai browser is wonderful, but exhibits some issues when used repeatedly.</p>
<ul>
  <li>Sign-in is required for full access on the Civitai site</li>
  <li>The user has to sift through menus to download a specific model</li>
  <li>A web browser is required</li>
  <li>File downloads contain little or no useful metadata, such as how the model/LoRA is supposed to be used (Safetensor metadata is often lacking)</li>
  <li>Other tools that utilize Civitai exist, such as some plugins for [WebUI](https://github.com/AUTOMATIC1111/stable-diffusion-webui), but none I could find have provided a CLI</li>
  <li>Many frontends I have found for Stable Diffusion seem to be needlessly complicated to install</li>
</ul>
<p>I wanted a CLI to do this for a number of reasons:</p>
<ul>
  <li>Speed</li>
  <li>Programmability/Scripting</li>
  <li>Simplicity</li>
  <li>Modularity, and usability as a library</li>
</ul>
<br>
## Roadmap
- [x] Help menu
- [x] Command-line options
- [x] CivitAI downloading
- [x] CivitAI querying
- [x] Interactive downloading (pick download from list)
- [ ] Manpage
- [ ] General Linux/WSL install script
- [ ] More ways to query and download (such as by Id)
- [ ] ArchLinux package
- [ ] Debian package
- [ ] Homebrew package for MacOS
- [ ] Better tests
- [ ] HuggingFace integration
<h2>Current State</h2>
<p>I have waited until this project is in a usable, (mostly) presentable state to make it public. I found a couple similar projects on crates.io, but those seem to have been abandoned.</p>
<h2>Boring Legal Stuff</h2>
<p>Copyright (c) 2024 Matthew M. Mitchell</p>
<p>Vorpal is available under the Apache or MIT license, depending on what you want.</p>
<p>Licenses can be found in the LICENSE-APACHE and LICENSE-MIT files, respectively.</p>
