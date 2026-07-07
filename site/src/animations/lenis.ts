import Lenis from '@studio-freight/lenis'

const lenis = new Lenis({
  duration: 1.2,
  smoothWheel: true,
  wheelMultiplier: 1,
  touchMultiplier: 2,
})

function raf(time:number){
    lenis.raf(time)
    requestAnimationFrame(raf)
}

requestAnimationFrame(raf)

export default lenis