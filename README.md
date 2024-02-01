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
<p><strong>Important note: </strong> This is a pre-release version. Bugs and issues may occur, and testing is not yet fully fleshed out. Check issues for more detail. These installation instructions are a temporary measure because I am having issues with cross-compilation for binaries.</p>
<p>Cargo is a <strong>temporary</strong> solution for installation. I want there to be at least something for people to use, should anyone want to use this.</p>
<br>
<h3>Linux, MacOS, and Windows (via WSL)</h3>
<p>Ensure you have the latest version of the [Rust](https://www.rust-lang.org/tools/install) language</p>
<p>Use this command to install Rust on Linux, MacOS, or Unix-likes (BSD)</p>

```
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

<br>
<p>(Recommended) navigate to a directory of your choice. I prefer a 'builds' directory in my home</p>


```
        mkdir $HOME/builds && cd $HOME/builds
```

<br>
<p>Clone the vorpal repo</p>

```
        git clone https://github.com/dekamicchin/vorpal.git
```

<br>
<p>Install Vorpal</p>

```
        cargo install
```

<br>
<p>Add cargo executables to your PATH so you can use it like you would a normal command-line program</p>

```
        echo 'export PATH=$HOME/.cargo/bin:$PATH' >> $HOME/.bash_profile
```

<br>
<p>MacOS users or zsh enjoyers: add the PATH adjustment to your corresponding shell startup</p>

```
        echo 'export PATH=$HOME/.cargo/bin:$PATH' >> $HOME/.zshrc
```

<br>
<p>Homebrew, Arch, and Debian packages are incoming, just please give me time.</p>
<br>
<h2>About</h2>
<p>Vorpal is lightweight, fast, and programmable. It is suitable for a multitude of environments, such as desktops or servers. You can easily find and download whatever model you need through easy to use and familiar command-line controls. Vorpal is also suitable for servers and enterprises, and allows machines to download models themselves without the need for rsync, ssh file transfers, or bloated git repos.</p>
<p>I want to focus on the downloading aspect, and make it as easy as possible to find and download models of more types and sources.</p>
<p>I built this because the current way of downloading models is annoying. The Civitai browser is wonderful, but exhibits some issues when used repeatedly.</p>
<ul>
  <li>Sign-in is required for full access on the Civitai site</li>
  <li>The user has to sift through menus to download a specific model</li>
  <li>A web browser is required</li>
  <li>File downloads contain little or no useful metadata, such as how the model/LoRA is supposed to be used (Safetensor metadata is often lacking)</li>
  <li>Other tools that utilize Civitai exist, such as some plugins for [WebUI](https://github.com/AUTOMATIC1111/stable-diffusion-webui) , but none I could find have provided a CLI</li>
  <li>Many frontends I have found for Stable Diffusion seem to be needlessly complicated to install</li>
</ul>
<br>
<p>I wanted a CLI to do this for a number of reasons:</p>
<ul>
  <li>Speed</li>
  <li>Programmability/Scripting</li>
  <li>Simplicity</li>
  <li>Modularity, and usability as a library</li>
</ul>
<br>
<h2>Usage/Examples</h2>
<p>
Usage: vorpal [OPTIONS] [MODEL_NAME]

Arguments:
  [MODEL_NAME]  The name of the model to download. First result will be downloaded

Options:
  -g, --get-first              Run in get-first mode (download first model from query)
  -d, --directory <DIRECTORY>  Specify a directory to download to. Overrides MODEL_DIRECTORY environment variable. Currnet directory will be used if both are empty
  -o, --only-model             Only download model (don't save metadata)
  -m, --meta                   Only get metadata of model
  -q, --query <QUERY>          Search Civitai for available models and LoRAs
  -c, --count <COUNT>          How many models to search [default: 15]
  -s, --safe                   Enter query as 'safe' (no NSFW)
  -f, --full                   Show full descriptions of query
  -u, --url <MODEL_NAME>       Return the download url of a model only
  -h, --help                   Print help
  -V, --version                Print version
</p>
<br>
<p>Search for models to download that match the query 'cat' (gives interactive menu to pick from)</p>

```
        vorpal cat
```

<br>
<p>Get (download model and metadata) first search result that comes from 'cat'</p>

```
        vorpal -g cat
```

<br>
<p>Search for models that match 'glitter'</p>

```
        vorpal -s -f glitter -c 3
```

<br>
<p>The -s option enters the query as 'safe'</p>
<p>The -f option tells vorpal to display the full descriptions (these can be long)</p>
<p>The -c option specifies how many results will be returned in the query API call</p>
<br>
<p>Search for cat models to download</p>

```
        MODEL_DIRECTORY=/home/me/my_models vorpal cat
```

<br>
<p>The -d option can be used which directory to download to instead of an environment variable</p>

```
        vorpal cat -d ~/home/my_models
```

<p>The -d option is meant to be used as an override, such as for downloading base SDXL checkpoints that usually belong in a separate folder. It is recommended to set a MODEL_DIRECTORY environment variable to make things easier for CLI use.</p>
<br>
<p>Other user-specified environment variables are also a good idea, like in this example:</p>

```
        echo 'export MODEL_DIRECTORY=/home/me/stable-diffusion-webui/models/Lora' >> ~/.zshrc
        echo 'export SDXL_CHECKPOINT_DIR=/home/me/stable-diffusion-webui/models/Stable-diffusion' >> ~/.zshrc
        source ~/.zshrc
        vorpal realcartoon -d SDXL_CHECKPOINT_DIR
```

<p>This is a simple example of setting and using environment variables to easily download models and loras to the desired locations.</p>
<br>


<br>
<h2>Roadmap</h2>

- [x] Help menu
- [x] Command-line options
- [x] CivitAI downloading
- [x] CivitAI querying
- [x] Interactive downloading (pick download from list)
- [ ] Manpage
- [X] General Linux/WSL install script
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
