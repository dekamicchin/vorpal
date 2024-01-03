<h1>Vorpal, a razor sharp AI model downloader CLI</h1>
<p>Vorpal is a small, rust-based, command-line utility to download Stable Diffusion models and LoRAs from Civitai (and potentially Huggingface in the future).  Searching and downloading the latest Stable Diffusion models and LoRAs can be painless and lightning-fast.</p>
<br>
<p>I built this because the current way of downloading models is annoying. The Civitai browser is wonderful, but exhibits some issues when used repeatedly.</p>
<ul>
  <li>Sign-in is required for full access on the Civitai site</li>
  <li>The user has to sift through or use filters to find a specific model</li>
  <li>A web browser is required</li>
  <li>File downloads contain little or no useful metadata, such as how the model/LoRA is supposed to be used</li>
  <li>Other tools that utilize Civitai exist, such as some plugins for [WebUI](https://github.com/AUTOMATIC1111/stable-diffusion-webui), but none I could find have provided a CLI</li>
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
  <li>All of the above, in that order, in mere seconds.</li>
  <li>Moves at the speed of Rust (that is to say, extremely fast)</li>
</ul>
<p> </p>
<h1> </h1>
