const formUrl =
  "https://forms.office.com/Pages/ResponsePage.aspx?id=fHiHmTGx-E69rTXjt8Mj7BpLVJXSV49OkPMHOvLlLddUNUJLOU5KTTNHVUdVS0k1QlQ1V0RXUFdTUi4u";

const submit = (e) => {
  e.preventDefault();

  const email = document.getElementById("email").value;

  if (email) {
    fetch(`/submit?email=${email}`, {
      method: "PUT",
    })
      // replaced for development purposes
      // .then((res) => window.location.replace(formUrl));
      .then((res) => console.log(res));
  }
};

var form = document.getElementById("form");
form.addEventListener("submit", submit);
