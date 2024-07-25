import init, { gen_elements, bind_event } from './hw2.js';
(async () => {
    await init();
    gen_elements();
    try {
        bind_event();
    } catch (error) {
        console.log(error);
    }
})();
