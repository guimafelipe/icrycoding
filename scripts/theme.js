let theme_button = document.querySelector("#tt");

console.log("loaded");

theme_button.addEventListener("click", (event) => {
	console.log("Clicked");
	document.body.classList.toggle("dark_mode");
});
