// const div = document.getElementById("Display 1");
// const observer = new MutationObserver(() => {div.scrollTop = div.scrollHeight;});
// observer.observe(div, {childList: true, subtree: true});
addEventListener("keydown", (event) => {
    if (event.code == "Enter") {
        candidates = document.getElementsByClassName("inline_input");
        console.log(window.getSelection());
    }
});
var display = document.getElementsByClassName("previous_lines")[0];
console.log(display);
display.addEventListener("DOMSubtreeModified", () => {console.log("Changed text");})