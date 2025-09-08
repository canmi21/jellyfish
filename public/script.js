const container = document.getElementById("dots")
const maxDots = 25
const dots = []

function createDot() {
  const dot = document.createElement("div")
  dot.classList.add("dot")
  dot.style.left = Math.random() * window.innerWidth + "px"
  dot.style.top = Math.random() * window.innerHeight + "px"
  dot.style.opacity = Math.random() * 0.6 + 0.2
  dot.style.transform = `scale(${Math.random() * 0.8 + 0.4})`
  dot.style.animationDuration = Math.random() * 3 + 2 + "s"
  dot.style.animationDelay = Math.random() * 2 + "s"
  container.appendChild(dot)
  dots.push({ el: dot, vx: (Math.random() - 0.5) * 0.25, vy: (Math.random() - 0.5) * 0.5 })
}

function updateDots() {
  dots.forEach(dot => {
    const rect = dot.el.getBoundingClientRect()
    let x = rect.left + dot.vx
    let y = rect.top + dot.vy
    if (x < -10) x = window.innerWidth + 10
    if (x > window.innerWidth + 10) x = -10
    if (y < -10) y = window.innerHeight + 10
    if (y > window.innerHeight + 10) y = -10
    dot.el.style.left = x + "px"
    dot.el.style.top = y + "px"
  })
  requestAnimationFrame(updateDots)
}

for (let i = 0; i < maxDots; i++) {
  createDot()
}
updateDots()

window.addEventListener("resize", () => {
  dots.forEach(dot => {
    dot.el.style.left = Math.random() * window.innerWidth + "px"
    dot.el.style.top = Math.random() * window.innerHeight + "px"
  })
})