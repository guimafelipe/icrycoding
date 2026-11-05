const str_theme_name = "icc_theme"
const str_dark = "dark_mode"

const theme = localStorage.getItem(str_theme_name);
const darkThemeMq = window.matchMedia("(prefers-color-scheme: dark)");

if (theme != null) {
	if (theme == "dark") {
		document.body.classList.add(str_dark);
	} else {
		document.body.classList.remove(str_dark);
	}
} else {
	if (darkThemeMq.matches) {
		document.body.classList.add(str_dark);
	} else {
		document.body.classList.remove(str_dark);
	}
}

const theme_button = document.querySelector("#tt");

theme_button.addEventListener("click", (event) => {
	console.log("Clicked");
	document.body.classList.toggle(str_dark);

	if (document.body.classList.contains(str_dark)) {
		localStorage.setItem(str_theme_name, "dark");
	} else {
		localStorage.setItem(str_theme_name, "light");
	}
});

