# CV Maker

Simple and free CV Maker that renders minimalistic CV.

Based on [idimetrix/cv](https://github.com/idimetrix/cv), simplified and ported to Rust.

<details>
<summary>This is what CV looks like:</summary>

![cv-builder](https://github.com/user-attachments/assets/6f656b1e-dfce-4429-b787-8f35414d1bc2)
</details>

All personal information is stored in a single `info.toml` file.
Images and icons are in the `static/images` folder.
Templates use Jinja syntax and are stored in `templates` folder.

To run the project, use:
```
cargo run
```
This starts an Actix server on http://localhost:3000/ that serves a CV in plain HTML.
This page can be printed to PDF, yielding you a proper CV.

All changes to the `info.toml` and templates are hot reloaded, so you can tweak them without restarting the server.


# Deployment as a personal GitHub Page

* Create and set up `your-username/your-username.github.io` repository as described in
  [GitHub Pages documentation](https://docs.github.com/en/pages/getting-started-with-github-pages/creating-a-github-pages-site).
  * For the workflow to work for the free GitHub account, this repository must be public.
  * Note that entire content of it will be overwritten every time.
* Create a personal token (**Settings** -> **Developer Settings** -> **Personal access tokens** > **Fine-grained token**)
  * This token must have repository access to the repository you've just created, with **write** access to **Contents**.
  * Better set it to **No expiration** because whatever. Otherwise, make sure to update it regularly before it expires. 
* Fork this CV repo.
  * Create a `data` branch in your fork, update `info.toml` there and add your images to `static/images`, the usual stuff.
  * Go to the **Actions** tab in your new fork. You will likely see a warning that workflows are disabled.
    Click the button to **Enable Workflows**.
* Go to **Settings** > **Secrets and variables** > **Actions**
  * **Secrets** > **Repository secrets**: `PERSONAL_TOKEN = (the personal token you created earlier)`
  * **Variables** > **Repository variables**: `TARGET_REPO = your-username/your-username.github.io`.
* Now, every push to the `data` branch will trigger a GitHub Action that will build the CV and push it to your
  GitHub Pages repository.
  Those changes should become visible `https://your-username.github.io` in a few minutes.
