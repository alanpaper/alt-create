title: 对vue3用法的吐槽
category: doc
<!-- -----split----- -->

#### 对vue3的一点吐槽

前些年，用过进1年vue2+, 后面react16.8出来以后，就没有在进一步接触vue，也就偶尔用vue写写小程序。

近期因为“项目”需要着手基于vue3开发一款基于electron的window桌面端应用。在之前也看过一些vue3的反应式api，没用之前感觉和hook差不多。语法方面也有很多相近，调研阶段甚至看到了用jsx写vue3。但是在实际开发中，遇到了很多自我感觉不是很简洁的语法。写过很久的react的实在无法忍受。

vue2阶段，一个props属性，在组件里我们可以像下面这么调用
```ts
...,
props: ["title", "name"]

method: {
  handleClick() {
    console.log(this.title)
  }
}

...,

```
这样我们就可以在方法里面基于vm.属性就可以访问。当到了vue3，我们需要下面这样

```ts
...,
props: ["title", "name"]
setup(props) {

  const { title, name } = toRefs(props)


  const handleClick = () => {

    console.log(title.value)
  }
}
...,

```
vue3中想要响应式,全要加一堆的特定语法,包括定义一个state需要用到`ref` or `reactive`等方法，当然react中有`useState()`。react中也有props。下面稍微做一个对比。

```ts

...

const { title, name } = props;

const handleClick = () => {
  console.log(title);
}

...

```
干脆升级，我还要写更多的代码量。 ---- 不能忍之一

下面看state。基于react的useState():

```ts
...

  const [title, setTitle] = useState<string>("");

  const handleClick = () => {
    setTitle("你最珍贵~");
  }

...


```
再来看基于vue3；

```ts
...

  const title = ref("")

  const handleClick => () => {
    title.value = "你最珍贵~";
  }

  // 重要不能少！
  return {
    title,
  }

...

```

这里关于定义状态的虽说代码量差不多，刚开始写vue3的我来说，复杂的业务逻辑，我做个判断我居然不能直接用变量。必须要用.value格式。我真的吐了。比如像下面这样：

```ts
...

  const online = ref(true);


  const handleClick = () => {
    if(online) {
      ... // 做了某些造作
      online = false
    }
  }

...

```
这个判断用过的都知道使用的有问题。但是真的一个业务组件，不知道要用state做多少判断。每一个state没做一次判断我居然要多些5个字母。我知道`ref`,`toRefs`是为了响应式。但是我是真的会忘记写.value。这个语法。。。。。我只能给零分。包括一还有一堆为了满足响应式而出现的新api。最后我调用vuex，想要响应式也要用到.value。 我....

下面是函数回调：

vue2中：

```ts
...

  method: {
    handleClick() {
      this.$emit("setName", "你最珍贵~")；
    }
}

...

```
我还能勉强接受。下面是vue3中：

```ts
...
  emit:["setName"],

  setup(props, content){

    const handleClick = () => {
      content.emit("setName", "你最珍贵~")
    }

  }

...

```

别告诉我，新api你不用, 但是我真的用不起，抱歉，还是选react和angular。还有个不想吐槽，但是还是拿出来说下


我写个组件，组件中简单用个slot替代父级传过来的dom,多个我可以加name，我说这么感觉写的很顺手的，结果我用组件的时候, 我的天,一堆template是什么鬼, 我写组件是为了简化代码的,结果我还是要写一堆的'样式代码'纯粹的'样式代码'。

真的用不起vue3,拜拜您来。。。





