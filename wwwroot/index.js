import init, {set_canvas, draw_canvas } from './rust_withoutcanvas.js';

let pressed = false;
const myXY = [];
const mainImg = document.getElementById("mainImg")
mainImg.addEventListener("dragstart", (event) => {
    event.preventDefault();
});
mainImg.addEventListener("mousedown", (event) => {
    pressed = true;
});
mainImg.addEventListener("mousemove", (event) => {
    const myRect = mainImg.getBoundingClientRect();
    const canvas_x = event.clientX - myRect.left;
    const canvas_y = event.clientY - myRect.top;
    //Canvas外
    if (canvas_x < 0 || canvas_x > mainImg.clientWidth || canvas_y < 0 || canvas_y > mainImg.clientHeight) {
        pressed = false;
    }
    if(pressed === true)
    {
        myXY.push({ x: canvas_x, y: canvas_y });
    }
});

mainImg.addEventListener("touchmove", (event) => {
    event.preventDefault();
    const myRect = mainImg.getBoundingClientRect();
    let canvas_x = event.touches[0].clientX - myRect.left;
    let canvas_y = event.touches[0].clientY - myRect.top;
    myXY.push({ x: canvas_x, y: canvas_y });
}, { passive: false });


async function run() {
    await init();
    document.getElementById("mainImg").src  = set_canvas();

    mainImg.addEventListener("mouseup", (event) => {
        console.log(myXY)
        document.getElementById("mainImg").src  = draw_canvas(mainImg.src, JSON.stringify(myXY));
        pressed = false;
        myXY.length = 0;
    });
    mainImg.addEventListener("touchend", (event) => {
        document.getElementById("mainImg").src  = draw_canvas(mainImg.src, JSON.stringify(myXY));
        myXY.length = 0;
    });

}
run();
