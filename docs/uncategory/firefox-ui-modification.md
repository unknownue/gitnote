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
    /* requires that you set
    * toolkit.legacyUserProfileCustomizations.stylesheets = true
    * in about:config
    */
    #mainPopupSet ~ box { -moz-box-ordinal-group: 10; }
    #titlebar { -moz-box-ordinal-group: 11; }
    #urlbar {
        top: unset !important;
        bottom: calc((var(--urlbar-toolbar-height) - var(--urlbar-height)) / 2) !important;
        box-shadow: none !important;
        display: flex !important;
        flex-direction: column !important;
    }
    #urlbar-input-container {
        order: 2;
    }
    #urlbar > .urlbarView {
        order: 1;
        border-bottom: 1px solid #666;
    }
    #urlbar-results {
        display: flex;
        flex-direction: column-reverse;
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
