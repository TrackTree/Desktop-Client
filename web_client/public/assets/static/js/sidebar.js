const template = document.createElement("template");

template.innerHTML = `
    <style>
        @import url("../css/variables.css");

        .logo-img {
            width: 3%;
            background: var(--black-level-2);
            padding: 3px;
            border-radius: 50%;
        }
        
        nav {
            background: var(--black-level-3);
            height: 100vh;
            width: 350px;
            padding: 1rem;
        }
    </style>

    <nav class="side-bar">
        <div class="logo">
            <img class="logo-img" src="./assets/images/icons/android-chrome-512x512.png" alt="logo">
        </div>
    </nav>
`;

class SideBar extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.shadowRoot.appendChild(template.content.cloneNode(true));
    this.innerHTML = "It works!";
  }
}

window.customElements.define("side-bar", SideBar);
