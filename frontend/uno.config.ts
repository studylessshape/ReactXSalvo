import { defineConfig, presetWind3, presetIcons, presetAttributify } from 'unocss';

export default defineConfig({
  // ...UnoCSS options
  presets: [
    presetWind3(),
    presetIcons(),
    presetAttributify(),
  ],
  outputToCssLayers: true,
});