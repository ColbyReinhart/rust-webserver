// Navbar setup
document.querySelector("#homepageLink").addEventListener("click", () => {
	window.location.assign("/");
});
document.querySelector("#resumeLink").addEventListener("click", () => {
	window.location.assign("/resume");
});
document.querySelector("#aboutmeLink").addEventListener("click", () => {
	window.location.assign("/aboutme");
});
document.querySelector("#projectsLink").addEventListener("click", () => {
	window.location.assign("/projects");
});
document.querySelector("#experienceLink").addEventListener("click", () => {
	window.location.assign("/experience");
});

// Hamburger menu toggle (only for mobile)
document.querySelector("#hamburgerMenu").addEventListener("click", () => {
	let navMenu = document.querySelector("#navbarLinks");
	if (navMenu.style.display == "none" || navMenu.style.display == "")
	{
		navMenu.style.display = "flex";
		navMenu.classList.remove("navHide");
		navMenu.classList.add("navShow");
	}
	else
	{
		navMenu.classList.remove("navShow");
		navMenu.classList.add("navHide");
		setTimeout(() => {navMenu.style.display="none";}, 700);
	}
});