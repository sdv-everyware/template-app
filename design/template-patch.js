// template-patch.js: the screen of template-app: the time, a title, a number and a button of the
// Library that counts up. Every control is a slot
// of a Library variant; the app (packages/services/display_safety/utils/template-app) drives the
// #template/* names. Applied to the EveryWare Library .fig, then ui-lib's dark-theme-patch.js
// for the dark Library:
//
//   design/build.sh     # the Library, this patch, the dark theme, data/template.dcf

// roundtrip.js hands over fig-helpers.js (the node authoring) as the second argument
module.exports = function (msg, { key, C, FONT, authoring }) {
  const A = authoring(msg, 'template');
  const { nodes, kids, frame, text, instance, solid, log } = A;
  // the Library's .fig (ui-lib design/build-library.sh): the screen goes on its Page 1
  const page = nodes.find(n => n.type === 'CANVAS' && n.name === 'Page 1');
  if (!page) throw new Error('no Page 1: build on the EveryWare Library .fig');
  if (nodes.some(n => n.name === '#template-root')) throw new Error('#template-root exists already');
  const variant = (set, name) => {
    const s = nodes.find(n => n.isStateGroup && n.name === set);
    const v = s && kids(key(s.guid)).find(n => n.type === 'SYMBOL' && n.name === name);
    if (!v) throw new Error(`no variant ${set} / ${name}`);
    return v;
  };
  // a slot: a frame named for the app with a preview instance of the variant inside; harry
  // clones the variant the code picks into it (ui-lib docs/content/index.md)
  const slot = (parent, name, x, y, v) => {
    const f = frame(parent, name, x, y, v.size.x, v.size.y, {
      bordersTakeSpace: false, stackMode: 'HORIZONTAL', stackPrimaryAlignItems: 'CENTER', stackCounterAlignItems: 'CENTER',
      stackPrimarySizing: 'FIXED', stackCounterSizing: 'FIXED', stackLayoutVersion: 1,
    });
    instance(f, `${name}/preview`, 0, 0, v);
    return f;
  };

  // ---- the screen: the Library's page colour, which the dark theme turns dark
  const root = frame(page, '#template-root', -2400, 4200, 1920, 1080, { fillPaints: [solid(C.page)], frameMaskDisabled: false });

  // ---- the time, top right: written by the code once a second
  text(root, '#template/time', 1600, 44, 224, 64, '20:42', 52, FONT.semibold, C.ink, 'RIGHT');

  // ---- the middle: a title of the design, the number the code writes, the button that counts
  text(root, 'Title', 360, 300, 1200, 96, 'Hello, EveryWare', 80, FONT.semibold, C.ink, 'CENTER');
  text(root, '#template/count', 660, 440, 600, 150, '0', 120, FONT.semibold, C.ink, 'CENTER');
  text(root, 'Caption', 660, 600, 600, 36, 'taps on the button', 28, FONT.regular, C.ink2, 'CENTER');
  const button = variant('#button', 'kind=primary, state=default, size=regular');
  slot(root, '#template/tap', 960 - button.size.x / 2, 700, button);

  log(`#template-root: ${A.count()} nodes`);
  return msg;
};
