import gsap from "gsap";
import { ScrollTrigger } from "gsap/ScrollTrigger";

gsap.registerPlugin(ScrollTrigger);

export function initAnimations(): void {
  hero();
  sections();

  // Recalculate trigger positions after everything is rendered
  ScrollTrigger.refresh();
}

/* -------------------------------- Hero -------------------------------- */

function hero() {
  const tl = gsap.timeline({
    defaults: {
      ease: "power3.out",
      duration: 0.8,
    },
  });

  tl.from("nav", {
    y: -50,
    opacity: 0,
  })
    .from(
      "header span",
      {
        y: 20,
        opacity: 0,
      },
      "-=0.4"
    )
    .from(
      "header h1",
      {
        y: 40,
        opacity: 0,
      },
      "-=0.3"
    )
    .from(
      "header p",
      {
        y: 25,
        opacity: 0,
      },
      "-=0.45"
    );
}

/* ------------------------------ Sections ------------------------------ */

function sections() {
  const reveal = [
    "#compress",
    "#motive",
    "#benchmarks",
    "footer",
  ];

  reveal.forEach((selector) => {
    const el = document.querySelector(selector);

    if (!el) return;

    gsap.from(el, {
      y: 60,
      opacity: 0,
      duration: 0.9,
      ease: "power2.out",
      scrollTrigger: {
        trigger: el,
        start: "top 80%",
        once: true,
      },
    });
  });

  // Peer Reviews (3rd section)
  const team = document.querySelectorAll("#team");

  if (team.length) {
    gsap.from(team, {
      y: 40,
      opacity: 0,
      stagger: 0.15,
      duration: 0.7,
      ease: "power2.out",
      scrollTrigger: {
        trigger: team[0],
        start: "top 85%",
        once: true,
      },
    });
  }
}