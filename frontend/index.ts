let pressKeyForm = document.querySelector<HTMLFormElement>("form#pressKey")

pressKeyForm?.addEventListener("submit", function (event) {
    event.preventDefault() // Prevent the default form submission

    const inputValue =
        document.querySelector<HTMLInputElement>("input#key")?.value ?? ""

    // Send POST request using Fetch API
    fetch(`/api/press/${inputValue}`, {
        method: "GET",
    })
        .then((response) => response.json())
        .then((data) => {
            console.log("Response from server:", data)
        })
        .catch((error) => {
            console.error("Error:", error)
        })
})
