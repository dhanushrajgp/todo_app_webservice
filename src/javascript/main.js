
let createButton = document.getElementById("create-button");
    createButton.addEventListener("click", postAlert);
    function postAlert() {
      let titleInput = document.getElementById("name");
      alert(titleInput.value);
      titleInput.value = null;
    }