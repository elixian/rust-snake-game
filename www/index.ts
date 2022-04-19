import init, { World, Direction } from "snake_game";

init().then(wasm => {

    const CELL_SIZE = 20;
    const WORLD_WIDTH = 16;
    const SNAKE_SPAWN_IDX = Date.now() % (WORLD_WIDTH ** 2);

    const world = World.new(WORLD_WIDTH, SNAKE_SPAWN_IDX);

    const worldWidth = world.width();
    const canvas = <HTMLCanvasElement>document.getElementById("snake-game");
    const ctx = canvas.getContext("2d");

    
    canvas.height = worldWidth * CELL_SIZE;
    canvas.width = worldWidth * CELL_SIZE;

    document.addEventListener("keydown", (e) => {
        switch (e.code) {
            case 'ArrowRight':
                world.change_dir(Direction.Right);
                break;
            case 'ArrowLeft':
                world.change_dir(Direction.Left);
                break;
            case 'ArrowUp':
                world.change_dir(Direction.Up);
                break;
            case 'ArrowDown':
                world.change_dir(Direction.Down);
                break;

            default:
                break;
        }
        console.log("test");
    })
    function drawWorld() {
        ctx.beginPath();

        for (let x = 0; x < worldWidth + 1; x++) {
            ctx.moveTo(CELL_SIZE * x, 0);
            ctx.lineTo(CELL_SIZE * x, CELL_SIZE * worldWidth)
        }

        for (let y = 0; y < worldWidth + 1; y++) {
            ctx.moveTo(0, CELL_SIZE * y);
            ctx.lineTo(CELL_SIZE * worldWidth, CELL_SIZE * y)
        }
        ctx.stroke();
    }

    function drawSnake() {
       
        const m = new Uint32Array(wasm.memory.buffer,world.snake_cells(),world.snake_len())
    
        m.forEach((x,i) =>{
            
            const col = x % worldWidth;
            const row = Math.floor(x / worldWidth);
            ctx.beginPath();
            if(i == 0){
                ctx.fillStyle= "#da2121";
            }else{
                ctx.fillStyle = "#459B65";
            }
            ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
            
            
           
        })

        ctx.stroke();
    }

    function drawCell(){
        let cell = world.reward_cells();

        const col  = cell % worldWidth;
        const row = Math.floor(cell / worldWidth);
        ctx.beginPath();
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
        ctx.stroke();
    }

    function paint() {
        drawWorld();
        drawCell();
        drawSnake();
    }

    function update() {
        world.step();
    }

    function initGame() {
        const FPS = 5;
        setTimeout(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            paint();
            update();

            requestAnimationFrame(initGame)
        }, 1000 / FPS)
    }

    initGame();
})