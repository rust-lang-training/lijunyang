(async () => {
    const { default: init, add, markdown2html } = await import('./hw.js');
    await init();
    const resetBtn = document.getElementById('reset');
    const convertBtn = document.getElementById('convert');
    const markInput = document.getElementById('mark-text');
    const htmlInput = document.getElementById('html-text');
    resetBtn.addEventListener('click', () => {
        markInput.value = '';
        htmlInput.value = '';
    });
    convertBtn.addEventListener('click', () => {
        htmlInput.value = markdown2html(markInput.value);
    });
    console.log(add(1, 2));
    console.log(markdown2html('# Hello World'));
})();
