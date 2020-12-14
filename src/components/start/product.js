
class Products{
    render() {
        let htmlCatalog = '';
        
    CATALOG.foreach(({id, name, price, img})=>{
      htmlCatalog += `
        <li>
          <span>${name}</span>
          <img src="${img}" />
          <span>${price}</span>
          <button>Добавить в корзину</button>
        </li>
        `;
    });
  
    const html = `
            <ul>
                ${htmlCatalog}
            </ul>
            `;
    }y
  }
  const productsPage = new Products();
  productsPage.render();