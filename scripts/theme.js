const theme_button = document.querySelector("#tt");

console.log("loaded");

theme_button.addEventListener("click", (event) => {
	console.log("Clicked");
	document.body.classList.toggle("dark_mode");

	if (document.body.classList.contains("dark_mode")) {
		localStorage.setItem("icc_theme", "dark");
	} else {
		localStorage.setItem("icc_theme", "light");
	}
});

const theme = localStorage.getItem("icc_theme");

if (theme != null) {
	if (theme == "dark") {
		document.body.classList.add("dark_mode");
	} else {
		document.body.classList.remove("dark_mode");
	}
}
