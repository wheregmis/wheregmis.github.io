/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'selector',
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      boxShadow: {
        neumorphic: '20px 20px 60px #d9d9d9, -20px -20px 60px #ffffff, 0 4px 6px -1px rgba(255, 255, 255, 0.1), 0 2px 4px -1px rgba(255, 255, 255, 0.06), 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)',
      },
      colors: {
        'background': '#121212', // Use for the background color of your application
        'on-background': '#FFFFFF', // Use for text and icons on top of the background color
        'primary': '#BB86FC', // Use for primary elements and call-to-action buttons
        'on-primary': '#000000', // Use for text and icons on top of the primary color
        'secondary': '#03A9F4', // Use for secondary elements and non-primary buttons
        'on-secondary': '#000000', // Use for text and icons on top of the secondary color
        'error': '#CF6679', // Use for error messages and icons
        'on-error': '#000000', // Use for text and icons on top of the error color
        'warning': '#FFA000', // Use for warning messages and icons
        'on-warning': '#000000', // Use for text and icons on top of the warning color
        'success': '#00C853', // Use for success messages and icons
        'on-success': '#000000', // Use for text and icons on top of the success color
        'info': '#03A9F4', // Use for informational messages and icons
        'on-info': '#000000', // Use for text and icons on top of the info color
      }
    },
  },
  plugins: [],
};
