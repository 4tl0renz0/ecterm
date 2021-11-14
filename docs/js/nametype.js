function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function typeECTerm() {
  progName.innerHTML = ">_";
  await sleep(850);
  progName.innerHTML = ">|";
  await sleep(250);
  progName.innerHTML = ">E|";
  await sleep(200);
  progName.innerHTML = ">EC|";
  await sleep(300);
  progName.innerHTML = ">ECT|";
  await sleep(200);
  progName.innerHTML = ">ECTe|";
  await sleep(250);
  progName.innerHTML = ">ECTer|";
  await sleep(300);
  progName.innerHTML = ">ECTerm|";
  await sleep(300);
  for (let i = 0; i < 2; i++) {
    progName.innerHTML = ">ECTerm_";
    await sleep(900);
    progName.innerHTML = ">ECTerm|";
    await sleep(900);
    i--
  }
}

window.onload = typeECTerm();
