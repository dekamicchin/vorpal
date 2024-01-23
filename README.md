<h1>Vorpal, a razor sharp AI model downloader CLI</h1>
<p>Vorpal is a small, rust-based, command-line utility to download Stable Diffusion models and LoRAs from Civitai (and potentially Huggingface, in the future).  Searching and downloading the latest Stable Diffusion models and LoRAs can be painless and lightning-fast.</p>
<br>
<p>I built this because the current way of downloading models is annoying. The Civitai browser is wonderful, but exhibits some issues when used repeatedly.</p>
<ul>
  <li>Sign-in is required for full access on the Civitai site</li>
  <li>The user has to sift through or use filters to find a specific model</li>
  <li>A web browser is required</li>
  <li>File downloads contain little or no useful metadata, such as how the model/LoRA is supposed to be used</li>
  <li>Other tools that utilize Civitai exist, such as some plugins for [WebUI](https://github.com/AUTOMATIC1111/stable-diffusion-webui), but none I could find have provided a CLI</li>
  <li>Many frontents I have found for Stable Diffusion seem to be needlessly complicated to install</li>
</ul>
<p>I wanted a CLI to do this for a number of reasons:</p>
<ul>
  <li>Speed</li>
  <li>Programmability/Scripting</li>
  <li>Simplicity</li>
  <li>Modularity, and usability as a library</li>
</ul>
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
<p>Vorpal is lightweight, fast, and programmable. It is suitable for a multitude of environments, such as desktops or servers. You can easily find and download whatever model you need through easy to use and familiar command-line controls. Vorpal is also suitable for servers and enterprises, and allows machines to download models themselves without the need for rsync, ssh file transfers, or bloated git repos.</p>
<p>I want to focus on the downloading aspect, and make it as easy as possible to find and download models of more types and sources.</p>
## Roadmap
- [x] Help menu
- [x] Command-line options
- [x] CivitAI downloading
- [x] CivitAI querying
- [x] Interactive downloading
- [ ] Manpage
- [ ] (Better) tests
<h2>Current State</h2>
<p>I have waited until this project is in a usable, (mostly) presentable state to make it public. I found a couple similar projects on crates.io, but those seem to have been abandoned.  </p>
<h1> </h1>
