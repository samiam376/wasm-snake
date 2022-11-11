import {ChangedCells, Universe} from 'wasm-snake';

const CELL_SIZE = 7; // px
const GRID_COLOR = "#CCCCCC";
const EMPTY = "#FFFFFF";
const SNAKE = "#000000";


const universe = Universe.new();
const width = universe.get_width();
const height = universe.get_height();

const canvas = document.getElementById("snake");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const score = document.getElementById("score");

let lastKeyStoke = undefined;

window.addEventListener('keydown', (event) => {
    event.preventDefault();
    if (event.key == 'ArrowUp') {
        lastKeyStoke = 0;
    }
    else if (event.code == 'ArrowDown') {
        lastKeyStoke=1;
    }
    else if (event.code == 'ArrowLeft') {
       lastKeyStoke = 2;
    }
    else if (event.code == 'ArrowRight') {
       lastKeyStoke = 3;
    }
})


const ctx = canvas.getContext('2d');

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
  
    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }
  
    // Horizontal lines.

    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }
  
    ctx.stroke();
};

const drawCells = (changed) => {
    if(changed === undefined){
        return
    }

    ctx.beginPath();
  

    for(let i = 0; i < changed.len; i++){
        const x = changed.xs[i];
        const y = changed.ys[i];
        const cell = changed.cells[i]
        const color = cell === 3 ? EMPTY : SNAKE;

        console.log(x, y, cell, color);
        ctx.fillStyle = color;
        ctx.fillRect(
            y * (CELL_SIZE + 1) + 1,
            x * (CELL_SIZE + 1) + 1,
            CELL_SIZE,
            CELL_SIZE
            );
    }

    ctx.stroke();

}
const renderLoop = async () => {
    await new Promise(r => setTimeout(r, 50)); 
    const changedCells = universe.tick(lastKeyStoke);

    drawGrid();
    drawCells(changedCells);
    requestAnimationFrame(renderLoop);
    console.log(changedCells.score)

    score.textContent = "Score: " + changedCells.score
  };

drawGrid();
drawCells(undefined);
requestAnimationFrame(renderLoop);
