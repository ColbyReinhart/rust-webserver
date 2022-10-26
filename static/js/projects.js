document.querySelector("#symplyfyGalleryButton").addEventListener("click", () => {
	let gallery = document.querySelector("#symplyfyGallery");
	gallery.style.animationName = "screen-appear";
	gallery.style.display = "block";
});

document.querySelector("#symplyfyGalleryClose").addEventListener("click", () => {
	let gallery = document.querySelector("#symplyfyGallery");
	gallery.style.animationName = "screen-hide";
	setTimeout(() => {gallery.style.display = "none";}, 950);
});

document.querySelector("#netpongGalleryButton").addEventListener("click", () => {
	let gallery = document.querySelector("#netpongGallery");
	gallery.style.animationName = "screen-appear";
	gallery.style.display = "block";
});

document.querySelector("#netpongGalleryClose").addEventListener("click", () => {
	let gallery = document.querySelector("#netpongGallery");
	gallery.style.animationName = "screen-hide";
	setTimeout(() => {gallery.style.display = "none";}, 950);
});
