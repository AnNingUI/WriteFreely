import { LitElement, html } from "lit";
import { classMap } from "lit/directives/class-map.js";
import { responsiveStyles } from '../../assets/css/TestImgStyle';


interface ImgItem {
  src: string;
  test: string;
  isOpen: boolean;
}




export class TestImgs extends LitElement {

  static styles = [responsiveStyles];

  _canRecovery = false;

  imgs: ImgItem[] = [];

  private ImgStyle = (url: string) => `
    margin: 5px; 
    background-image: url(${url});
    background-size: cover;
    border-radius: 40px;
  `;

  render() {
    return html`
      <div class="container">
        ${this.imgs.map((item) => html`
          <div
            alt=${item.test}
            class=${classMap({ open: item.isOpen, close: !item.isOpen })}
            style=${this.ImgStyle(item.src)}
            @click=${() => this.toggleOpen(item)}
          >
            <h3 
              class="${item.isOpen ? "open" : "close"} test"
              style="width: fit-content; height: fit-content; padding: 2px; background-color: rgba(0, 0, 0, 0.014); border-radius: 5px;"
            >${item.test}</h3>
          </div>
        `)}
      </div>
    `;
  };

  toggleOpen(item: ImgItem) {
    if (item.isOpen && !this._canRecovery) return;
    const isOpen = !item.isOpen;
    this.imgs = this.imgs.map(i => ({ ...i, isOpen: i === item ? isOpen : false }));
    this.requestUpdate();
  }
}





