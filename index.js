const rust = import('./pkg/virtualworld');
const canvas = document.getElementById("virtualWorld");

rust.then(m => {
    if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
        canvas.height = window.innerHeight;
        canvas.clientHeight = window.innerHeight;
        canvas.style.height = window.innerHeight;

        canvas.width = window.innerWidth;
        canvas.clientWidth = window.innerWidth;
        canvas.style.width = window.innerWidth;
    }
    
    m.start()
})








