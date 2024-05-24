title: 基于hooks实现表单组件化
category: doc
<!-- -----split----- -->

### 前端实现动态表单

    缘起：项目中很多页面是可查询表格页面；后端由于历史原因又无法给每个页面添加自定义的查询表单元数据的接口；于是有了下面的前端的处理方式

项目代码基于 antd-design 的 form 组件,当然其他的也可以自行修改使用,通过 hooks 获取后端对应的字典数据并处理，eg：处理表单查询联动等问题

首先，定义定义 form 表单的元数据(可自行按需扩展)

```ts
// 元数据类型定义
interface IFormItem {
  name: string;
  label: string;
  type: "input" | "select" | "multiple" | "checked" | "radio";
  onChange?: (e: any) => void;
  options?: { value: number | string; label: string }[];
}
```

然后根据相应的类型定义 searchForm 组件;

### 表单元数据拼接

每个 hooks 对应一个或多个(需要联动的话)后端的字典数据，然后结合页面需求对个个 hooks 进行拼接组合，生成 form 元数据，传入 searchForm 组件即可。

```tsx
// 通过hooks获取远程数据也可自定义数据eg:
const useSeachYearAndMonth = (form) => {

  const years = [{value:2023,label: "2023年"}, {value:2022, label: "2022年"}]；
  const [month, setMonth] = useState<{name:number, label: string}[]>([])

  const handleChangeYear = (e:any) => {
    const months = [1,2,3,4,5,6,7,8,9,10,11,12];
    const lastMonth = Math.floor(Math.random() * 10);
    setMonth(months.slice(0,lastMonth).map(item=>({
      value: item,
      label: `${item}月`
    })));
    form.setFieldValue({
      ...form.getFieldsVlaue(),
      month:null
    })
  }


  const config:IFormItem[] = [
    {
      name:"year",
      label:"年份",
      type:"select",
      onChange: handleChangeYear,
      options: years
    }, {
      name: "month",
      label: "月份",
      type:"select",
      options: month,
    }
  ]

  return config;
}


const useSearchName = (form, option?: {name?: string, label?:string, type: string}) => {
  return [{
    name: option?.name || "name",
    label: option?.label || "姓名",
    type: option?.type || "input",
  }]
}


```

#### 使用

在相关页面的第一个层级使用一下方式，组合个个查询条件。

```tsx
// 拼接元数据传给form组件即可
const formDataSource: IFormItem[] = [
  {
    name: "name",
    label: "姓名",
    type: "input",
  },
  ...useSearchName(form),
  ...useSeachYearAndMondth(form),
];
```
