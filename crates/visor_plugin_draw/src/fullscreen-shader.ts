export class FullscreenShader {
  #id: number;

  constructor(id: number) {
    this.#id = id;
  }

  id() {
    return this.#id;
  }
}
