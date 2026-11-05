const str_theme_name = "icc_theme"; // What I save in localStorage
const str_dark = "dark_mode"; // Class defined in CSS

const theme = localStorage.getItem(str_theme_name);
const darkThemeMq = window.matchMedia("(prefers-color-scheme: dark)");

if ((theme == "dark") || (theme == null && darkThemeMq.matches)){
	document.body.classList.add(str_dark);
}

const theme_button = document.querySelector("#tt");

theme_button.addEventListener("click", (event) => {
	document.body.classList.toggle(str_dark);

	if (document.body.classList.contains(str_dark)) {
		localStorage.setItem(str_theme_name, "dark");
	} else {
		localStorage.setItem(str_theme_name, "light");
	}
});

