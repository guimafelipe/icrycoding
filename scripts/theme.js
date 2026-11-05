const str_theme_name = "icc_theme"; // What I save in localStorage
const str_dark = "dark_mode"; // Class defined in CSS

const theme = localStorage.getItem(str_theme_name);
const darkThemeMq = window.matchMedia("(prefers-color-scheme: dark)");

if ((theme == "dark") || (theme == null && darkThemeMq.matches)){
	document.querySelector("html").classList.add(str_dark);
}

