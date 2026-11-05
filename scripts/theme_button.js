const theme_button = document.querySelector("#tt");

theme_button.addEventListener("click", (event) => {
	document.querySelector("html").classList.toggle(str_dark);

	if (document.querySelector("html").classList.contains(str_dark)) {
		localStorage.setItem(str_theme_name, "dark");
	} else {
		localStorage.setItem(str_theme_name, "light");
	}
});

