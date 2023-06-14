import { Config } from "tailwindcss"
import daisyui from "daisyui"

export default {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  plugins: [daisyui],
  daisyui: {
    themes: ["cyberpunk", "night"],
    darkTheme: "night",
  }
} satisfies Config;