# json5_se

a very simple module for stringify javascript object to json in python with rust extensions

## simple benchmark

```python
import json5
import json5_se

data_small = """{
  // comments
  unquoted: 'and you can quote me on that',
  singleQuotes: 'I can use "double quotes" here',
  lineBreaks: "Look, Mom! \
No \\n's!",
  hexadecimal: 0xdecaf,
  leadingDecimalPoint: .8675309, andTrailing: 8675309.,
  positiveSign: +1,
  trailingComma: 'in objects', andIn: ['arrays',],
  "backwardsCompatible": "with JSON",
}"""

%timeit json5.loads(data_small)
2.59 ms ± 17.7 µs per loop (mean ± std. dev. of 7 runs, 100 loops each)

%timeit json.loads(json5_se.loads(data_small))
521 µs ± 5.28 µs per loop (mean ± std. dev. of 7 runs, 1000 loops each)

```

```python
// not that large
data_large = {
    "localSizeList": {
        "category_id": 0,
        "supplier_category_id": 0,
        "skc": "ss2106294239470202",
        "size_rule_list": [],
        "country_code": "",
        "category_name": "",
        "goods_id": 3107316
    },
    "getSellingPoints": [],
    "wishGood": 0,
    "soldoutColor": [],
    "tsp": [
        {
            "c7d_top_checkout_uv": "",
            "goods_id": "3107316",
            "premium_series_id": "",
            "current_top_browsing_uv": "1124",
            "brand": "",
            "c7d_top_wishlist_uv": "6070"
        }
    ],
    "multiLocalSize": {
        "category_id": 0,
        "supplier_category_id": 0,
        "skc": "ss2106294239470202",
        "size_rule_list": [],
        "category_name": "",
        "goods_id": 3107316
    },
    "goods_imgs": {
        "main_image": {
            "origin_image": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974248b69bcf376c754fb0f863f911a90fd6.jpg",
            "thumbnail": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974248b69bcf376c754fb0f863f911a90fd6_thumbnail_220x293.jpg",
            "medium_image": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974248b69bcf376c754fb0f863f911a90fd6_thumbnail_405x552.jpg",
            "thumbnail_webp": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974248b69bcf376c754fb0f863f911a90fd6_thumbnail_220x293.webp",
            "origin_image_webp": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974248b69bcf376c754fb0f863f911a90fd6.webp",
            "medium_image_webp": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974248b69bcf376c754fb0f863f911a90fd6_thumbnail_405x552.webp"
        },
        "detail_image": [
            {
                "origin_image": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974425614c5f2ca89e30c254fafba523bf34.jpg",
                "medium_image": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974425614c5f2ca89e30c254fafba523bf34_thumbnail_405x552.jpg",
                "thumbnail": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974425614c5f2ca89e30c254fafba523bf34_thumbnail_220x293.jpg",
                "thumbnail_webp": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974425614c5f2ca89e30c254fafba523bf34_thumbnail_220x293.webp",
                "origin_image_webp": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974425614c5f2ca89e30c254fafba523bf34.webp",
                "medium_image_webp": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974425614c5f2ca89e30c254fafba523bf34_thumbnail_405x552.webp"
            },
            {
                "origin_image": "//img.ltwebstatic.com/images3_pi/2021/06/30/1625019746529256dbea94959f9b2688cbad6e9ada.jpg",
                "medium_image": "//img.ltwebstatic.com/images3_pi/2021/06/30/1625019746529256dbea94959f9b2688cbad6e9ada_thumbnail_405x552.jpg",
                "thumbnail": "//img.ltwebstatic.com/images3_pi/2021/06/30/1625019746529256dbea94959f9b2688cbad6e9ada_thumbnail_220x293.jpg",
                "thumbnail_webp": "//img.ltwebstatic.com/images3_pi/2021/06/30/1625019746529256dbea94959f9b2688cbad6e9ada_thumbnail_220x293.webp",
                "origin_image_webp": "//img.ltwebstatic.com/images3_pi/2021/06/30/1625019746529256dbea94959f9b2688cbad6e9ada.webp",
                "medium_image_webp": "//img.ltwebstatic.com/images3_pi/2021/06/30/1625019746529256dbea94959f9b2688cbad6e9ada_thumbnail_405x552.webp"
            }
        ],
        "video_url": ""
    },
    "detail": {
        "goods_id": "3107316",
        "cat_id": "1964",
        "goods_sn": "ss2106294239470202",
        "goods_url_name": "Chili Shaped Ballpoint Pen",
        "supplier_id": "2002641",
        "goods_name": "Chili Shaped Ballpoint Pen",
        "original_img": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974248b69bcf376c754fb0f863f911a90fd6.jpg",
        "is_stock_enough": "1",
        "goods_thumb": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974248b69bcf376c754fb0f863f911a90fd6_thumbnail_220x293.jpg",
        "goods_img": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974248b69bcf376c754fb0f863f911a90fd6_thumbnail_405x552.jpg",
        "brand": "",
        "sizeTemplate": [],
        "goods_desc": "WARNING: CHOKING HAZARD-Small parts, not for children under 3 yrs.",
        "supplier_top_category_id": "2",
        "parent_id": "3654",
        "is_on_sale": "1",
        "is_virtual_stock": "0",
        "stock": "999",
        "is_init": "1",
        "is_pre_sale": "0",
        "is_pre_sale_end": "0",
        "isMultiPartProduct": 0,
        "multiPartInfo": [],
        "mainSaleAttrShowMode": 2,
        "productDetails": [
            {
                "attr_id": 1000096,
                "attr_value_id": "112",
                "attr_name": "Refill Color",
                "attr_name_en": "Refill Color",
                "value_sort": 0,
                "attr_select": 1,
                "attr_sort": 0,
                "left_show": 1,
                "attr_value": "Black",
                "attr_value_en": "Black",
                "attr_desc": "",
                "attr_image": ""
            },
            {
                "attr_id": 220,
                "attr_value_id": "2415",
                "attr_name": "Refill Specifications",
                "attr_name_en": "Refill Specifications",
                "value_sort": 0,
                "attr_select": 1,
                "attr_sort": 0,
                "left_show": 1,
                "attr_value": "0.5mm",
                "attr_value_en": "0.5mm",
                "attr_desc": "",
                "attr_image": ""
            },
            {
                "attr_id": 109,
                "attr_value_id": "1000036",
                "attr_name": "Type",
                "attr_name_en": "Type",
                "value_sort": 0,
                "attr_select": 1,
                "attr_sort": 0,
                "left_show": 1,
                "attr_value": "Ballpoint pen",
                "attr_value_en": "Ballpoint pen",
                "attr_desc": "",
                "attr_image": ""
            },
            {
                "attr_id": 160,
                "attr_value_id": "517",
                "attr_name": "Material",
                "attr_name_en": "Material",
                "value_sort": 0,
                "attr_select": 1,
                "attr_sort": 80,
                "left_show": 1,
                "attr_value": "Plastic",
                "attr_value_en": "Plastic",
                "attr_desc": "",
                "attr_image": ""
            }
        ],
        "mainSaleAttribute": [
            {
                "attr_id": 1000096,
                "attr_value_id": "112",
                "attr_name": "Refill Color",
                "attr_name_en": "Refill Color",
                "value_sort": 0,
                "attr_select": 1,
                "attr_sort": 0,
                "left_show": 1,
                "attr_value": "Black",
                "attr_value_en": "Black",
                "attr_desc": "",
                "attr_image": ""
            }
        ],
        "comment": {
            "comment_num": "0",
            "comment_rank": "0"
        },
        "is_subscription": "0",
        "promotionInfo": [],
        "promotion": null,
        "color_image": "//img.ltwebstatic.com/images3_pi/2021/06/30/16250197505dc3934e8170ffe5d9302fa4865d3cdc.jpg",
        "productRelationID": "s21062884662",
        "retailPrice": {
            "amount": "2.00",
            "amountWithSymbol": "US$2.00",
            "usdAmount": "2.00",
            "usdAmountWithSymbol": "US$2.00"
        },
        "goods_img_webp": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974248b69bcf376c754fb0f863f911a90fd6_thumbnail_405x552.webp",
        "original_img_webp": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974248b69bcf376c754fb0f863f911a90fd6.webp",
        "salePrice": {
            "amount": "2.00",
            "amountWithSymbol": "US$2.00",
            "usdAmount": "2.00",
            "usdAmountWithSymbol": "US$2.00"
        },
        "unit_discount": "0",
        "special_price_old": {
            "amount": "2.00",
            "amountWithSymbol": "US$2.00",
            "usdAmount": "2.00",
            "usdAmountWithSymbol": "US$2.00"
        },
        "is_clearance": 0,
        "limit_count": "7",
        "flash_goods": {
            "is_flash_goods": 0
        },
        "isPriceConfigured": 1,
        "appPromotion": [],
        "rewardPoints": 2,
        "doublePoints": 0,
        "color_type": "0",
        "beautyCategory": false,
        "needAttrRelation": false,
        "brandInfo": null,
        "series": null
    },
    "exclusivePromotionPrice": null,
    "country": "US",
    "sizeInfoDes": {
        "multiPartFlag": 0,
        "multiPartInfo": [],
        "sizeInfo": [
            {
                "attr_id": 87,
                "attr_name": "Size",
                "attr_value_id": 474,
                "attr_value_name": "one-size",
                "attr_value_name_en": "one-size",
                "Length ": " 13 cm",
                "Diameter ": " 1 cm"
            }
        ],
        "sizeInfoInch": [
            {
                "attr_id": 87,
                "attr_name": "Size",
                "attr_value_id": 474,
                "attr_value_name": "one-size",
                "attr_value_name_en": "one-size",
                "Length ": " 5.1 inch",
                "Diameter ": " 0.4 inch"
            }
        ],
        "sizeUnit": 1,
        "allcmFlag": 1,
        "sizeInfoAttribute": [],
        "basicAttribute": {
            "image_url": "",
            "attribute_info": [],
            "base_code_info": [],
            "base_code_info_inch": [],
            "base_code_info_other": [],
            "base_code_show_mode": 0
        },
        "braRecommendSize": []
    },
    "relation_color": [
        {
            "goods_id": "3017425",
            "cat_id": "1964",
            "goods_sn": "ss2106288466254923",
            "goods_url_name": "Carrot Shaped Ballpoint Pen",
            "supplier_id": "2002641",
            "goods_name": "Carrot Shaped Ballpoint Pen",
            "original_img": "//img.ltwebstatic.com/images3_pi/2021/06/29/16249452896d16b96e430613ec8cb6ea15b28e2ad7.jpg",
            "brand": "",
            "sizeTemplate": [],
            "is_init": "1",
            "stock": "999",
            "is_on_sale": "1",
            "is_virtual_stock": "0",
            "isMultiPartProduct": 0,
            "multiPartInfo": [],
            "mainSaleAttrShowMode": 2,
            "productDetails": [
                {
                    "attr_id": 1000096,
                    "attr_value_id": "112",
                    "attr_name": "Refill Color",
                    "attr_name_en": "Refill Color",
                    "value_sort": 0,
                    "attr_select": 1,
                    "attr_sort": 0,
                    "left_show": 1,
                    "attr_value": "Black",
                    "attr_value_en": "Black",
                    "attr_desc": "",
                    "attr_image": ""
                },
                {
                    "attr_id": 220,
                    "attr_value_id": "2415",
                    "attr_name": "Refill Specifications",
                    "attr_name_en": "Refill Specifications",
                    "value_sort": 0,
                    "attr_select": 1,
                    "attr_sort": 0,
                    "left_show": 1,
                    "attr_value": "0.5mm",
                    "attr_value_en": "0.5mm",
                    "attr_desc": "",
                    "attr_image": ""
                },
                {
                    "attr_id": 109,
                    "attr_value_id": "1000036",
                    "attr_name": "Type",
                    "attr_name_en": "Type",
                    "value_sort": 0,
                    "attr_select": 1,
                    "attr_sort": 0,
                    "left_show": 1,
                    "attr_value": "Ballpoint pen",
                    "attr_value_en": "Ballpoint pen",
                    "attr_desc": "",
                    "attr_image": ""
                },
                {
                    "attr_id": 160,
                    "attr_value_id": "517",
                    "attr_name": "Material",
                    "attr_name_en": "Material",
                    "value_sort": 0,
                    "attr_select": 1,
                    "attr_sort": 80,
                    "left_show": 1,
                    "attr_value": "Plastic",
                    "attr_value_en": "Plastic",
                    "attr_desc": "",
                    "attr_image": ""
                }
            ],
            "mainSaleAttribute": [
                {
                    "attr_id": 1000096,
                    "attr_value_id": "112",
                    "attr_name": "Refill Color",
                    "attr_name_en": "Refill Color",
                    "value_sort": 0,
                    "attr_select": 1,
                    "attr_sort": 0,
                    "left_show": 1,
                    "attr_value": "Black",
                    "attr_value_en": "Black",
                    "attr_desc": "",
                    "attr_image": ""
                }
            ],
            "promotionInfo": [],
            "promotion": null,
            "relatedColor": [
                {
                    "goods_id": "3017425",
                    "goods_relation_id": "s21062884662",
                    "cat_id": "1964",
                    "goods_sn": "ss2106288466254923",
                    "goods_url_name": "Carrot Shaped Ballpoint Pen",
                    "goods_name": "Carrot Shaped Ballpoint Pen",
                    "goods_color_image": "//img.ltwebstatic.com/images3_pi/2021/06/29/162494529942ba5329ca46fe7d9ad720f8d651b3f0.jpg",
                    "original_img": "//img.ltwebstatic.com/images3_pi/2021/06/29/16249452896d16b96e430613ec8cb6ea15b28e2ad7.jpg",
                    "goods_thumb": "//img.ltwebstatic.com/images3_pi/2021/06/29/16249452896d16b96e430613ec8cb6ea15b28e2ad7_thumbnail_220x293.jpg",
                    "goods_img": "//img.ltwebstatic.com/images3_pi/2021/06/29/16249452896d16b96e430613ec8cb6ea15b28e2ad7_thumbnail_405x552.jpg"
                },
                {
                    "goods_id": "3107316",
                    "goods_relation_id": "s21062884662",
                    "cat_id": "1964",
                    "goods_sn": "ss2106294239470202",
                    "goods_url_name": "Chili Shaped Ballpoint Pen",
                    "goods_name": "Chili Shaped Ballpoint Pen",
                    "goods_color_image": "//img.ltwebstatic.com/images3_pi/2021/06/30/16250197505dc3934e8170ffe5d9302fa4865d3cdc.jpg",
                    "original_img": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974248b69bcf376c754fb0f863f911a90fd6.jpg",
                    "goods_thumb": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974248b69bcf376c754fb0f863f911a90fd6_thumbnail_220x293.jpg",
                    "goods_img": "//img.ltwebstatic.com/images3_pi/2021/06/30/162501974248b69bcf376c754fb0f863f911a90fd6_thumbnail_405x552.jpg"
                }
            ],
            "parent_id": "3654",
            "supplier_top_category_id": "2",
            "color_image": "//img.ltwebstatic.com/images3_pi/2021/06/29/162494529942ba5329ca46fe7d9ad720f8d651b3f0.jpg",
            "productRelationID": "s21062884662",
            "retailPrice": {
                "amount": "2.00",
                "amountWithSymbol": "US$2.00",
                "usdAmount": "2.00",
                "usdAmountWithSymbol": "US$2.00"
            },
            "salePrice": {
                "amount": "2.00",
                "amountWithSymbol": "US$2.00",
                "usdAmount": "2.00",
                "usdAmountWithSymbol": "US$2.00"
            },
            "unit_discount": "0",
            "is_clearance": 0,
            "limit_count": "7",
            "flash_goods": {
                "is_flash_goods": 0
            },
            "isPriceConfigured": 1,
            "rewardPoints": 2,
            "doublePoints": 0,
            "color_type": "0",
            "beautyCategory": false,
            "needAttrRelation": false,
            "comment": {
                "comment_num": "0",
                "comment_rank": "0"
            },
            "is_subscription": "0",
            "is_pre_sale": "0",
            "is_pre_sale_end": "0",
            "is_stock_enough": "0",
            "goods_thumb": "//img.ltwebstatic.com/images3_pi/2021/06/29/16249452896d16b96e430613ec8cb6ea15b28e2ad7_thumbnail_220x293.jpg",
            "goods_img": "//img.ltwebstatic.com/images3_pi/2021/06/29/16249452896d16b96e430613ec8cb6ea15b28e2ad7_thumbnail_405x552.jpg",
            "goods_img_webp": "//img.ltwebstatic.com/images3_pi/2021/06/29/16249452896d16b96e430613ec8cb6ea15b28e2ad7_thumbnail_405x552.webp",
            "original_img_webp": "//img.ltwebstatic.com/images3_pi/2021/06/29/16249452896d16b96e430613ec8cb6ea15b28e2ad7.webp"
        }
    ],
    "gradeState": [
        {
            "goods_id": 3107316,
            "skc": "ss2106294239470202",
            "new_product_unsale": 0
        }
    ],
    "cccTemplateData": {
        "appTemplate": 1,
        "content": [
            {
                "detailImg": "1",
                "sizeChart": "1",
                "description": "1",
                "countCell": "1",
                "imgSrc": "",
                "colorType": "2"
            }
        ],
        "id": 148266,
        "sourceId": 236756,
        "tempCode": null,
        "tempType": 1,
        "templateId": 46
    },
    "getSeriesAndBrand": {
        "skc": "ss2106294239470202",
        "goods_id": 3107316,
        "brand": null,
        "series": null
    },
    "rule_id": 0,
    "more_goods_imgs": [],
    "getOtherOptions": [],
    "getBeautyGoodsDesc": {
        "goodsDesc": "",
        "goodsComponent": "",
        "beautyCategory": 0
    },
    "hotColorList": [
        {
            "hot_color": "0",
            "goods_id": "3107316",
            "new_arrival_28": "0"
        },
        {
            "hot_color": "0",
            "goods_id": "3017425",
            "new_arrival_28": "0"
        }
    ],
    "skcSort": {
        "ss2106288466254923": 0,
        "ss2106287865165266": 0,
        "ss2106296105814607": 0,
        "ss2106298202227187": 0,
        "ss2106293991941939": 0,
        "ss2106294239470202": 0
    },
    "commentInfo": {
        "spu": "s21062884662",
        "goods_id": null,
        "comment_num": 521,
        "comment_rank_average": "4.88",
        "percent_overall_fit": {
            "true_size": "100%",
            "large": "0%",
            "small": "0%"
        }
    },
    "getItemPlusSize": {},
    "currentCat": {
        "cat_id": "1964",
        "site_id": "7",
        "url_cat_id": "1964",
        "cat_url_name": "Pens & Refills",
        "goods_type_id": "0",
        "show_in_nav": "1",
        "image": "",
        "image_app": "",
        "parent_id": "3654",
        "sort_order": "0",
        "is_leaf": "1",
        "is_return": "1",
        "cat_name": "Pens & Refills",
        "meta_title": "",
        "meta_keywords": "",
        "meta_description": "",
        "cat_desc": "",
        "category_description_seo": "",
        "left_description": "",
        "language_flag": "en"
    },
    "metaInfo": {
        "meta_title": "Chili Shaped Ballpoint Pen | SHEIN USA",
        "meta_description": "Free Returns \\u2713 Free Shipping On Orders $49+ \\u2713. Chili Shaped Ballpoint Pen- Pens & Refills at SHEIN.",
        "meta_keywords": ""
    },
    "parentCats": {
        "cat_id": "2297",
        "url_cat_id": "2297",
        "goods_type_id": "0",
        "cat_url_name": "Office & School Supplies",
        "cat_name": "Office & School Supplies",
        "parent_id": "0",
        "sort_order": "0",
        "is_leaf": "0",
        "goods_num": "0",
        "multi": {
            "cat_id": "2297",
            "cat_name": "Office & School Supplies",
            "meta_title": "",
            "meta_keywords": "",
            "meta_description": "",
            "cat_desc": "",
            "language_flag": "en"
        },
        "children": [
            {
                "cat_id": "3654",
                "url_cat_id": "3654",
                "goods_type_id": "0",
                "cat_url_name": "Writing Supplies",
                "cat_name": "Writing Supplies",
                "parent_id": "2297",
                "sort_order": "0",
                "is_leaf": "0",
                "goods_num": "0",
                "multi": {
                    "cat_id": "3654",
                    "cat_name": "Writing Supplies",
                    "meta_title": "",
                    "meta_keywords": "",
                    "meta_description": "",
                    "cat_desc": "",
                    "language_flag": "en"
                },
                "children": [
                    {
                        "cat_id": "1964",
                        "url_cat_id": "1964",
                        "goods_type_id": "0",
                        "cat_url_name": "Pens & Refills",
                        "cat_name": "Pens & Refills",
                        "parent_id": "3654",
                        "sort_order": "0",
                        "is_leaf": "1",
                        "goods_num": "0",
                        "multi": {
                            "cat_id": "1964",
                            "cat_name": "Pens & Refills",
                            "meta_title": "",
                            "meta_keywords": "",
                            "meta_description": "",
                            "cat_desc": "",
                            "language_flag": "en"
                        },
                        "children": []
                    }
                ]
            }
        ]
    },
    "model": {},
    "attrSizeList": {
        "sale_attr_list": {
            "3107316": {
                "goods_id": "3107316",
                "goods_sn": "ss2106294239470202",
                "skc_name": "ss2106294239470202",
                "sku_list": [
                    {
                        "sku_code": "I7n2odabpk2g",
                        "stock": 999,
                        "sku_sale_attr": [
                            {
                                "attr_id": 87,
                                "attr_name": "Size",
                                "attr_name_en": "Size",
                                "attr_value_id": 474,
                                "attr_value_name": "one-size",
                                "attr_value_name_en": "one-size"
                            }
                        ],
                        "price": {
                            "retailPrice": {
                                "amount": "2.00",
                                "amountWithSymbol": "US$2.00",
                                "usdAmount": "2.00",
                                "usdAmountWithSymbol": "US$2.00"
                            },
                            "salePrice": {
                                "amount": "2.00",
                                "amountWithSymbol": "US$2.00",
                                "usdAmount": "2.00",
                                "usdAmountWithSymbol": "US$2.00"
                            },
                            "unit_discount": 0
                        },
                        "rewardPoints": 2,
                        "doublePoints": 0
                    }
                ],
                "skc_sale_attr": [
                    {
                        "attr_id": 87,
                        "attr_name": "Size",
                        "attr_value_list": [
                            {
                                "attr_value_id": 474,
                                "attr_value_name": "one-size",
                                "attr_value_name_en": "one-size",
                                "attr_std_value": ""
                            }
                        ]
                    }
                ]
            }
        },
        "attrSize": []
    },
    "afterPayInfo": {
        "id": 21,
        "is_display": 1,
        "payment_code": "afterpay-card",
        "show_desc": "",
        "place_order_desc": "or 4 interest-free payments of US$0.50",
        "jump_url": "https://us.shein.com/Afterpay-a-1007.html",
        "place_order_jump_url": "https://us.shein.com/Afterpay-a-1007.html",
        "logo_url": "http://img.ltwebstatic.com/images3_pi/2020/09/23/16008290258c6e56705475b8df7cde96a8218e28ec.png",
        "decimal_rule": 1,
        "stage": 4,
        "show_desc_batch": [
            {
                "amount": "2.00",
                "desc": ""
            }
        ],
        "front_show_list": [
            {
                "id": 21,
                "is_display": 1,
                "payment_code": "afterpay-card",
                "show_desc": "",
                "place_order_desc": "or 4 interest-free payments of US$0.50",
                "jump_url": "https://us.shein.com/Afterpay-a-1007.html",
                "place_order_jump_url": "https://us.shein.com/Afterpay-a-1007.html",
                "logo_url": "http://img.ltwebstatic.com/images3_pi/2020/09/23/16008290258c6e56705475b8df7cde96a8218e28ec.png",
                "decimal_rule": 1,
                "stage": 4,
                "show_desc_batch": [
                    {
                        "amount": "2.00",
                        "desc": ""
                    }
                ]
            },
            {
                "id": 123,
                "is_display": 1,
                "payment_code": "klarna-sliceitdirect",
                "show_desc": "",
                "place_order_desc": "Make 4 payments of US$0.50",
                "jump_url": "https://us.shein.com/Klarna-a-1011.html",
                "place_order_jump_url": "https://us.shein.com/Klarna-a-1011.html",
                "logo_url": "http://img.ltwebstatic.com/images3_pi/2021/06/09/16232213583b12c66e1f14764516fb8dd209ac3c46.png",
                "decimal_rule": 2,
                "stage": 4,
                "show_desc_batch": [
                    {
                        "amount": "2.00",
                        "desc": ""
                    }
                ]
            },
            {
                "id": 157,
                "is_display": 1,
                "payment_code": "zip-quadpay",
                "show_desc": "",
                "place_order_desc": "4 interest-free payments of US$0.50",
                "jump_url": "https://us.shein.com/Zip-a-1101.html",
                "place_order_jump_url": "https://us.shein.com/Zip-a-1101.html",
                "logo_url": "http://img.ltwebstatic.com/images3_pi/2021/08/10/1628582232b83c1a27cbf8df7f40473a5a847014aa.png",
                "decimal_rule": 1,
                "stage": 4,
                "show_desc_batch": [
                    {
                        "amount": "2.00",
                        "desc": ""
                    }
                ]
            }
        ]
    }
}

%timeit json5.loads(data_large)
176 ms ± 2.77 ms per loop (mean ± std. dev. of 7 runs, 1 loop each)

%timeit json.loads(json5_se.loads(data_large))
30.1 ms ± 350 µs per loop (mean ± std. dev. of 7 runs, 10 loops each)
```


