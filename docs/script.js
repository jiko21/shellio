function dissolveInGenerate(doc) {
  return (entries) => {
    // if (entires[0].intersetionRatio === 0) return;
    doc.classList.add('section-animation');
  }
}

window.onload = () => {
  const docs = [
    document.getElementById('what-is-shellio'),
    document.getElementById('how'),
    document.getElementById('option'),
  ];
  const option = {
    threshold: 1,
  };
  docs.forEach((doc) => {
    const observer = new IntersectionObserver(dissolveInGenerate(doc), option);
    observer.observe(doc);
  })
};
