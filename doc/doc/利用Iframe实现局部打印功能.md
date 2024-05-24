title: 利用Iframe实现局部打印功能
category: doc
<!-- -----split----- -->

### 利用Iframe实现局部打印功能


#### 放在需要实现局部的页面内
```html
  <iframe width="1" height="1" id="printIframe" class="print-iframe" src="about:blank"></iframe>
```

```css
    .print-iframe {
      display: none;
      width: 1px;
      height: 1px;
    }
```


### 打印

```ts

  export function previewPdf(nativeElement, name: string) {

    const cloneHtml = nativeElement.cloneNode(true);

    const printIframe = document.getElementById("printIframe") as HTMLIFrameElement;

    const newPrintDoc = document.implementation.createHTMLDocument(name);

    try {
      const link = newPrintDoc.createElement("link");
      link.setAttribute("rel", "stylesheet");
      link.setAttribute("href", "../assets/print.css");
      newPrintDoc.head.appendChild(link);
      newPrintDoc.body.appendChild(cloneHtml);
    } catch(e) {
      console.log(e);
    }

    const iframeDocument= printIframe.contentDocument;
    const newPrintDocumentElement = newPrintDoc.documentElement;
    const newNode = iframeDocument.importNode(newPrintDocumentElement, true);
    iframeDocument.replaceChild(newNode, iframeDocument.documentElement);

    setTimeout(()=>{
      printIframe.contentWindow.print();
    },1000)

  }


```



