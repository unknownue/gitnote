---
tags: [Firefox]
title: Firefox UI Modification
created: '2020-01-09T08:49:04.060Z'
modified: '2020-01-15T08:58:27.388Z'
---

# Firefox UI Modification

Unlock custom CSS usage in Firefox 69 and newer

`about:config` > `toolkit.legacyUserProfileCustomizations.stylesheets` > `true`

Profile folders location on drive:
Windows
```
C:\Users\ USERNAME \AppData\Roaming\Mozilla\Firefox\Profiles\ PROFILE FOLDER NAME \
Hidden files must be visible to see AppData folder. Alternatively open %APPDATA%\Mozilla\Firefox\Profiles\ from explorers location bar.
```
Linux
```
/home/ username /.mozilla/firefox/ profile folder name /
Hidden files must be visible to see .mozilla folder.
```

Mac OS X
```
~\Library\Mozilla\Firefox\Profiles\ PROFILE FOLDER NAME \ or
~\Library\Application Support\Firefox\Profiles\ PROFILE FOLDER NAME \chrome or
\Users\ USERNAME \Library\Application\Support\Firefox\Profiles\
```
From https://github.com/Aris-t2/CustomCSSforFx

`userChrome.css`
```CSS

/* Move the tabbar and address bar below the page content */
@import "tabs_below_content.css"; /**/

/* Hide app button on macOS*/
@import "appbutton_hidden.css"; /**/
```

`tabs_below_content.css`
```CSS

/* Code from https://github.com/jonhoo/configs/blob/master/gui/.mozilla/firefox/dev-edition-default/chrome/userChrome.css */

@-moz-document url(chrome://browser/content/browser.xhtml) {
	/* tabs on bottom of window */
	/*
  we want to move #navigator-toolbox, but since FF72, it is wrapped in
  a `box` element that has no identifier. we would like to use:
  box:has(#navigator-toolbox) { -moz-box-ordinal-group: 10; }
  but :has isn't a "live selector", so we can't use it in stylesheet
  context. instead, we use this hack:
	*/
	#mainPopupSet ~ box { -moz-box-ordinal-group: 10; }
	#urlbar { -moz-box-ordinal-group: 11; }
	.urlbarView {
		top: unset !important;
		bottom: 61px !important;
		box-shadow: none !important;
	}
	.search-one-offs { display: none !important; }
	.tab-background { border-top: none !important; }
	#navigator-toolbox::after { border: none; }
	#TabsToolbar .tabbrowser-arrowscrollbox,
	#tabbrowser-tabs, .tab-stack { min-height: 28px !important; }
	.tabbrowser-tab { font-size: 80%; }
	.tab-content { padding: 0 5px; }
	.tab-close-button .toolbarbutton-icon { width: 12px !important; height: 12px !important; }
}
```

`appbutton_hidden.css`
```CSS

/* Hide min, max, close in title bar */
#TabsToolbar > .titlebar-buttonbox-container {
	display: none!important;
}
```


