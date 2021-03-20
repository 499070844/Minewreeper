import * as wasm from 'wasm-game'
const CELL_SIZE = 40;
const GRID_COLOR = "#CCCCCC";
const NORMAL_COLOR = "#000000";

let board =''
/**
 * 从 wasm 拿到 string 类型的棋盘，并绑定到全局变量里
 * # Bug
 * ```rs
 *  #[wasm_bindgen(module="[path]")] // 无法在rs中获取
 *  // path 尝试过 'index', '/www/index', '/www/dist/index', '/www/src/index'都不行
 * 
 * ```
 *  
 * @param s 
 */
export function render(s: string) {
  board = s
  console.info(board)
}
// render 函数要绑定 window, 可以通过 #[wasm_bin...(js_namespage = window)]获取
(window as any).render = render;

const test_closure = (cb: any) => {
  console.log('test1');
  cb();
}

const test2_closure = (cb: any) => {
  console.log('test2');
  cb();
}

(window as any).test2_closure = test2_closure;
(window as any).test_closure = test_closure;

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
  console.log(getIndex(X, Y));
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

class CounterBlock implements Drawable {
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
  drawText: CounterBlock
  drawBlock: Block
  constructor(ctx: CanvasRenderingContext2D) {
    this.drawBoom = new Boom(ctx);
    this.drawBlock = new Block(ctx);
    this.drawText = new CounterBlock(ctx);
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


/**
 * 画框框
 * @param ctx 
 */
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

/**
 * 画格子
 *  1. 普通格子 class Block
 *  2. 已翻转的安全格子（显示数字） class CounterBlock
 *  3. 已翻转的炸弹格子 (显示炸弹) class Boom
 * @param ctx 
 */
function drawBlock(ctx: CanvasRenderingContext2D) {
  ctx.beginPath();
  const baseView = new BaseCanvasView(ctx);
    for (let row = 0; row <= 8; row++) {
      for (let col = 0; col <= 8; col++) {
        const data = board[getIndex(row, col)]
        baseView.factory(data).draw(row, col, data);
      }
    }
}

if (canvas) {
  // mouse controll
  canvas.addEventListener('click', getPosition, false);
  canvas.height = (CELL_SIZE + 1) * 9 + 1;
  canvas.width = (CELL_SIZE + 1) * 9 + 1;
  let a = wasm.init();
  a.test();
  a.test();
  a.test();

  const ctx = canvas.getContext("2d")
  if (ctx) {
    drawGrid(ctx);
    drawBlock(ctx);
  }
}
