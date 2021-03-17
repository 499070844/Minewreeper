import * as wasm from 'wasm-game'
const CELL_SIZE = 40;
const GRID_COLOR = "#CCCCCC";
const NORMAL_COLOR = "#000000";

let a =''
export function render(s: string) {
  a = s
  console.info(a)
}
(window as any).render = render;




///////Canvas//////////////////

let canvas = document.querySelector<HTMLCanvasElement>("#canvas");

const getIndex = (x: number, y: number): number => {
  return x + y * 9
}

const getPosition = (event: MouseEvent) => {
  const mouseX = event.clientX - canvas!.offsetLeft;
  const mouseY = event.clientY - canvas!.offsetTop;
  const X = Math.ceil(mouseX / (CELL_SIZE + 1)) - 1;
  const Y = Math.ceil(mouseY / (CELL_SIZE + 1)) - 1;
}

interface Drawable {
  ctx: CanvasRenderingContext2D
  draw(x: number, y: number, text: string): void;
}
class Block implements Drawable {
  ctx: CanvasRenderingContext2D
  fillColor: string
  constructor(ctx: CanvasRenderingContext2D) { this.ctx = ctx; this.fillColor = NORMAL_COLOR }
  draw(x: number, y: number, text: string) {
    this.ctx.fillStyle = this.fillColor;
    this.ctx.fillRect(
      x * (CELL_SIZE + 1) + 1,
      y * (CELL_SIZE + 1) + 1,
      CELL_SIZE,
      CELL_SIZE,
      )
  }
}

class SafeBlock implements Drawable {
  ctx: CanvasRenderingContext2D
  constructor(ctx: CanvasRenderingContext2D) { this.ctx = ctx; }
  draw(x: number, y: number, text: string) {
    const row = x * (CELL_SIZE + 1) + 18;
    const col = y * (CELL_SIZE + 1) + 25;
    this.ctx.fillText(text, row, col, 20);
  }
}

class Boom implements Drawable {
  ctx: CanvasRenderingContext2D
  fillStyle: string
  constructor(ctx: CanvasRenderingContext2D) { this.ctx = ctx; this.fillStyle = "#D75455"; }
  draw(x: number, y: number, text: string) {
    this.ctx.fillStyle = this.fillStyle;
    this.ctx.fillRect(
      x * (CELL_SIZE + 1) + 1,
      y * (CELL_SIZE + 1) + 1,
      CELL_SIZE,
      CELL_SIZE,
    )
    this.ctx.fillStyle = NORMAL_COLOR;
    this.ctx.fillText(
      "✹",
      x * (CELL_SIZE + 1) + 18,
      y * (CELL_SIZE + 1) + 25,
      20
      );
  }
}

class BaseCanvasView {
  drawBoom: Boom
  drawText: SafeBlock
  drawBlock: Block
  constructor(ctx: CanvasRenderingContext2D) {
    this.drawBoom = new Boom(ctx);
    this.drawBlock = new Block(ctx);
    this.drawText = new SafeBlock(ctx);
  }
  factory(text: string): Drawable {
    switch (text) {
      case '✹':
        return this.drawBoom;
      case '■':
        return this.drawBlock;
      default:
        return this.drawText;
    }
  }
}



function drawGrid(ctx: CanvasRenderingContext2D) {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines
  for (let i = 0; i <= 9; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * 9 + 1);
  }
  // Horizontal lines
  for (let i = 0; i <= 9; i++) {
    ctx.moveTo(0, i * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * 9 + 1, i * (CELL_SIZE + 1) + 1);
  }
  ctx.stroke();
}

function drawBlock(ctx: CanvasRenderingContext2D) {
  ctx.beginPath();
  const baseView = new BaseCanvasView(ctx);
    for (let row = 0; row <= 8; row++) {
      for (let col = 0; col <= 8; col++) {
        const data = a[getIndex(row, col)]
        baseView.factory(data).draw(row, col, data);
      }
    }
}

if (canvas) {
  // mouse controll
  canvas.addEventListener('click', getPosition, false);
  canvas.height = (CELL_SIZE + 1) * 9 + 1;
  canvas.width = (CELL_SIZE + 1) * 9 + 1;
  wasm.main();

  const ctx = canvas.getContext("2d")
  if (ctx) {
    drawGrid(ctx);
    drawBlock(ctx);
  }
}
