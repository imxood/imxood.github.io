module.exports = {
    content: ['./src/routes/**/*.{svelte,js,ts}'],
    plugins: [require('daisyui')],
    corePlugins: {
        preflight: true,
    }
};
