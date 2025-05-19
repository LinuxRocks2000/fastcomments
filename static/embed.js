let template = `
<style>
._fastcomments_outer {
  border-top: 1px solid black;
  padding-top: 10px;
  margin-top: 10px;
}
._fastcomments_form {
  text-align: center;
}
._fastcomments_comment {
  border: 1px solid purple;
  padding: 10px;
  max-width: 1000px;
  margin-left: auto;
  margin-right: auto;
}
</style>
<div class="_fastcomments_form">
  <b>Post a comment</b><br />
  Username: <input type="text" id="_fastcomments_username" /><br/>
  Content: <textarea id="_fastcomments_content"></textarea><br/>
  <button onclick="post().then(rebuild)">Post</button>
</div>
<div class="_fastcomments_comments">

</div>
`;
let el = document.createElement("div");
el.classList.add("_fastcomments_outer");
el.innerHTML = template;
document.currentScript.parentNode.insertBefore(el, document.currentScript);

let comments_list_element = document.querySelector("._fastcomments_comments");

let target_host = (new URL(document.currentScript.src)).origin;

async function rebuild() {
  comments_list_element.innerHTML = "";
  let req = await fetch(target_host + "/comments/" + FASTCOMMENTS_PATH);
  let json = await req.json();
  json.forEach(item => {
    let el = document.createElement("div");
    el.classList.add("_fastcomments_comment");
    let content_el = document.createElement("p");
    content_el.innerText = item.content;
    el.innerHTML = `
<b>${item.username}</b><br>
    `;
    el.appendChild(content_el);
    comments_list_element.appendChild(el);
  });
}

async function post() {
  if (document.querySelector("#_fastcomments_content").value != "") {
    if (document.querySelector("#_fastcomments_username").value != "") {
      await fetch(target_host + "/post/", {
        method: "POST",
        body: JSON.stringify({
          "page": FASTCOMMENTS_PATH,
          "username": document.querySelector("#_fastcomments_username").value,
          "content": document.querySelector("#_fastcomments_content").value
        }),
        headers: {
          "Content-Type": "application/json"
        }
      });
      document.querySelector("#_fastcomments_content").value = "";
    }
    else {
      alert("username is required");
    }
  }
  else {
    alert("content is required");
  }
}

rebuild();
