var N = null;var sourcesIndex = {};
sourcesIndex["adler"] = {"name":"","files":["algo.rs","lib.rs"]};
sourcesIndex["adler32"] = {"name":"","files":["lib.rs"]};
sourcesIndex["bitflags"] = {"name":"","files":["lib.rs"]};
sourcesIndex["bumpalo"] = {"name":"","files":["alloc.rs","lib.rs"]};
sourcesIndex["bytemuck"] = {"name":"","files":["contiguous.rs","lib.rs","offset_of.rs","pod.rs","transparent.rs","zeroable.rs"]};
sourcesIndex["byteorder"] = {"name":"","files":["io.rs","lib.rs"]};
sourcesIndex["cfg_if"] = {"name":"","files":["lib.rs"]};
sourcesIndex["color_quant"] = {"name":"","files":["lib.rs","math.rs"]};
sourcesIndex["crc32fast"] = {"name":"","dirs":[{"name":"specialized","files":["mod.rs","pclmulqdq.rs"]}],"files":["baseline.rs","combine.rs","lib.rs","table.rs"]};
sourcesIndex["crossbeam_channel"] = {"name":"","dirs":[{"name":"flavors","files":["array.rs","at.rs","list.rs","mod.rs","never.rs","tick.rs","zero.rs"]}],"files":["channel.rs","context.rs","counter.rs","err.rs","lib.rs","select.rs","select_macro.rs","utils.rs","waker.rs"]};
sourcesIndex["crossbeam_deque"] = {"name":"","files":["deque.rs","lib.rs"]};
sourcesIndex["crossbeam_epoch"] = {"name":"","dirs":[{"name":"sync","files":["list.rs","mod.rs","queue.rs"]}],"files":["atomic.rs","collector.rs","default.rs","deferred.rs","epoch.rs","guard.rs","internal.rs","lib.rs"]};
sourcesIndex["crossbeam_utils"] = {"name":"","dirs":[{"name":"atomic","files":["atomic_cell.rs","consume.rs","mod.rs","seq_lock.rs"]},{"name":"sync","files":["mod.rs","parker.rs","sharded_lock.rs","wait_group.rs"]}],"files":["backoff.rs","cache_padded.rs","lib.rs","thread.rs"]};
sourcesIndex["ctor"] = {"name":"","files":["lib.rs"]};
sourcesIndex["deflate"] = {"name":"","files":["bit_reverse.rs","bitstream.rs","chained_hash_table.rs","checksum.rs","compress.rs","compression_options.rs","deflate_state.rs","encoder_state.rs","huffman_lengths.rs","huffman_table.rs","input_buffer.rs","length_encode.rs","lib.rs","lz77.rs","lzvalue.rs","matching.rs","output_writer.rs","rle.rs","stored_block.rs","writer.rs","zlib.rs"]};
sourcesIndex["either"] = {"name":"","files":["lib.rs"]};
sourcesIndex["ghost"] = {"name":"","files":["args.rs","derive.rs","lib.rs","parse.rs","variance.rs","visibility.rs"]};
sourcesIndex["gif"] = {"name":"","dirs":[{"name":"reader","files":["decoder.rs","mod.rs"]}],"files":["common.rs","encoder.rs","lib.rs","traits.rs"]};
sourcesIndex["image"] = {"name":"","dirs":[{"name":"codecs","dirs":[{"name":"bmp","files":["decoder.rs","encoder.rs","mod.rs"]},{"name":"hdr","files":["decoder.rs","encoder.rs","mod.rs"]},{"name":"ico","files":["decoder.rs","encoder.rs","mod.rs"]},{"name":"jpeg","files":["decoder.rs","encoder.rs","entropy.rs","mod.rs","transform.rs"]},{"name":"pnm","files":["autobreak.rs","decoder.rs","encoder.rs","header.rs","mod.rs"]},{"name":"tga","files":["decoder.rs","encoder.rs","header.rs","mod.rs"]},{"name":"webp","files":["decoder.rs","mod.rs","transform.rs","vp8.rs"]}],"files":["dds.rs","dxt.rs","farbfeld.rs","gif.rs","png.rs","tiff.rs"]},{"name":"imageops","files":["affine.rs","colorops.rs","mod.rs","sample.rs"]},{"name":"io","files":["free_functions.rs","mod.rs","reader.rs"]},{"name":"math","files":["mod.rs","nq.rs","rect.rs","utils.rs"]},{"name":"utils","files":["mod.rs"]}],"files":["animation.rs","buffer.rs","color.rs","dynimage.rs","error.rs","flat.rs","image.rs","lib.rs","traits.rs"]};
sourcesIndex["image_go_nord"] = {"name":"","files":["lib.rs","palette.rs","utils.rs"]};
sourcesIndex["indoc"] = {"name":"","files":["lib.rs"]};
sourcesIndex["indoc_impl"] = {"name":"","files":["lib.rs"]};
sourcesIndex["instant"] = {"name":"","files":["lib.rs","native.rs"]};
sourcesIndex["inventory"] = {"name":"","files":["lib.rs"]};
sourcesIndex["inventory_impl"] = {"name":"","files":["lib.rs"]};
sourcesIndex["itertools"] = {"name":"","dirs":[{"name":"adaptors","files":["coalesce.rs","map.rs","mod.rs","multi_product.rs"]}],"files":["combinations.rs","combinations_with_replacement.rs","concat_impl.rs","cons_tuples_impl.rs","diff.rs","either_or_both.rs","exactly_one_err.rs","format.rs","free.rs","group_map.rs","groupbylazy.rs","grouping_map.rs","impl_macros.rs","intersperse.rs","k_smallest.rs","kmerge_impl.rs","lazy_buffer.rs","lib.rs","merge_join.rs","minmax.rs","multipeek_impl.rs","pad_tail.rs","peek_nth.rs","peeking_take_while.rs","permutations.rs","powerset.rs","process_results_impl.rs","put_back_n_impl.rs","rciter_impl.rs","repeatn.rs","size_hint.rs","sources.rs","tee.rs","tuple_impl.rs","unique_impl.rs","with_position.rs","zip_eq_impl.rs","zip_longest.rs","ziptuple.rs"]};
sourcesIndex["itoa"] = {"name":"","files":["lib.rs"]};
sourcesIndex["jpeg_decoder"] = {"name":"","dirs":[{"name":"worker","files":["immediate.rs","mod.rs","multithreaded.rs"]}],"files":["decoder.rs","error.rs","huffman.rs","idct.rs","lib.rs","marker.rs","parser.rs","upsampler.rs"]};
sourcesIndex["lazy_static"] = {"name":"","files":["inline_lazy.rs","lib.rs"]};
sourcesIndex["libc"] = {"name":"","dirs":[{"name":"unix","dirs":[{"name":"linux_like","dirs":[{"name":"linux","dirs":[{"name":"arch","dirs":[{"name":"generic","files":["mod.rs"]}],"files":["mod.rs"]},{"name":"gnu","dirs":[{"name":"b64","dirs":[{"name":"x86_64","files":["align.rs","mod.rs","not_x32.rs"]}],"files":["mod.rs"]}],"files":["align.rs","mod.rs"]}],"files":["align.rs","mod.rs"]}],"files":["mod.rs"]}],"files":["align.rs","mod.rs"]}],"files":["fixed_width_ints.rs","lib.rs","macros.rs"]};
sourcesIndex["lock_api"] = {"name":"","files":["lib.rs","mutex.rs","remutex.rs","rwlock.rs"]};
sourcesIndex["log"] = {"name":"","files":["lib.rs","macros.rs"]};
sourcesIndex["memoffset"] = {"name":"","files":["lib.rs","offset_of.rs","raw_field.rs","span_of.rs"]};
sourcesIndex["memory_units"] = {"name":"","files":["lib.rs"]};
sourcesIndex["miniz_oxide"] = {"name":"","dirs":[{"name":"deflate","files":["buffer.rs","core.rs","mod.rs","stream.rs"]},{"name":"inflate","files":["core.rs","mod.rs","output_buffer.rs","stream.rs"]}],"files":["lib.rs","shared.rs"]};
sourcesIndex["num_cpus"] = {"name":"","files":["lib.rs","linux.rs"]};
sourcesIndex["num_integer"] = {"name":"","files":["average.rs","lib.rs","roots.rs"]};
sourcesIndex["num_iter"] = {"name":"","files":["lib.rs"]};
sourcesIndex["num_rational"] = {"name":"","files":["lib.rs","pow.rs"]};
sourcesIndex["num_traits"] = {"name":"","dirs":[{"name":"ops","files":["checked.rs","inv.rs","mod.rs","mul_add.rs","overflowing.rs","saturating.rs","wrapping.rs"]}],"files":["bounds.rs","cast.rs","float.rs","identities.rs","int.rs","lib.rs","macros.rs","pow.rs","real.rs","sign.rs"]};
sourcesIndex["parking_lot"] = {"name":"","files":["condvar.rs","deadlock.rs","elision.rs","fair_mutex.rs","lib.rs","mutex.rs","once.rs","raw_fair_mutex.rs","raw_mutex.rs","raw_rwlock.rs","remutex.rs","rwlock.rs","util.rs"]};
sourcesIndex["parking_lot_core"] = {"name":"","dirs":[{"name":"thread_parker","files":["linux.rs","mod.rs"]}],"files":["lib.rs","parking_lot.rs","spinwait.rs","util.rs","word_lock.rs"]};
sourcesIndex["paste"] = {"name":"","files":["lib.rs"]};
sourcesIndex["paste_impl"] = {"name":"","files":["enum_hack.rs","error.rs","lib.rs"]};
sourcesIndex["png"] = {"name":"","dirs":[{"name":"decoder","files":["mod.rs","stream.rs","zlib.rs"]}],"files":["chunk.rs","common.rs","encoder.rs","filter.rs","lib.rs","traits.rs","utils.rs"]};
sourcesIndex["proc_macro2"] = {"name":"","files":["detection.rs","fallback.rs","lib.rs","marker.rs","parse.rs","wrapper.rs"]};
sourcesIndex["proc_macro_hack"] = {"name":"","files":["error.rs","iter.rs","lib.rs","parse.rs","quote.rs"]};
sourcesIndex["pyo3"] = {"name":"","dirs":[{"name":"class","files":["basic.rs","buffer.rs","context.rs","descr.rs","gc.rs","iter.rs","macros.rs","mapping.rs","methods.rs","mod.rs","number.rs","proto_methods.rs","pyasync.rs","sequence.rs"]},{"name":"err","files":["err_state.rs","impls.rs","mod.rs"]},{"name":"ffi","dirs":[{"name":"cpython","files":["abstract_.rs","bytesobject.rs","ceval.rs","code.rs","dictobject.rs","frameobject.rs","listobject.rs","mod.rs"]}],"files":["bltinmodule.rs","boolobject.rs","bytearrayobject.rs","bytesobject.rs","ceval.rs","code.rs","codecs.rs","compile.rs","complexobject.rs","context.rs","datetime.rs","descrobject.rs","dictobject.rs","enumobject.rs","eval.rs","fileobject.rs","floatobject.rs","funcobject.rs","genobject.rs","import.rs","initconfig.rs","intrcheck.rs","iterobject.rs","listobject.rs","longobject.rs","marshal.rs","memoryobject.rs","methodobject.rs","mod.rs","modsupport.rs","moduleobject.rs","object.rs","objectabstract.rs","objimpl.rs","osmodule.rs","pyarena.rs","pycapsule.rs","pydebug.rs","pyerrors.rs","pyframe.rs","pyhash.rs","pylifecycle.rs","pymem.rs","pyport.rs","pystate.rs","pystrtod.rs","pythonrun.rs","rangeobject.rs","setobject.rs","sliceobject.rs","structmember.rs","structseq.rs","sysmodule.rs","traceback.rs","tupleobject.rs","typeslots.rs","unicodeobject.rs","warnings.rs","weakrefobject.rs"]},{"name":"types","files":["any.rs","boolobject.rs","bytearray.rs","bytes.rs","complex.rs","datetime.rs","dict.rs","floatob.rs","function.rs","iterator.rs","list.rs","mod.rs","module.rs","num.rs","sequence.rs","set.rs","slice.rs","string.rs","tuple.rs","typeobject.rs"]}],"files":["buffer.rs","callback.rs","conversion.rs","derive_utils.rs","exceptions.rs","freelist.rs","gil.rs","instance.rs","internal_tricks.rs","lib.rs","marshal.rs","once_cell.rs","panic.rs","prelude.rs","pycell.rs","pyclass.rs","pyclass_init.rs","pyclass_slots.rs","python.rs","type_object.rs"]};
sourcesIndex["pyo3_macros"] = {"name":"","files":["lib.rs"]};
sourcesIndex["pyo3_macros_backend"] = {"name":"","files":["defs.rs","from_pyobject.rs","konst.rs","lib.rs","method.rs","module.rs","proto_method.rs","pyclass.rs","pyfunction.rs","pyimpl.rs","pymethod.rs","pyproto.rs","utils.rs"]};
sourcesIndex["quote"] = {"name":"","files":["ext.rs","format.rs","ident_fragment.rs","lib.rs","runtime.rs","spanned.rs","to_tokens.rs"]};
sourcesIndex["rayon"] = {"name":"","dirs":[{"name":"collections","files":["binary_heap.rs","btree_map.rs","btree_set.rs","hash_map.rs","hash_set.rs","linked_list.rs","mod.rs","vec_deque.rs"]},{"name":"compile_fail","files":["cannot_collect_filtermap_data.rs","cannot_zip_filtered_data.rs","cell_par_iter.rs","mod.rs","must_use.rs","no_send_par_iter.rs","rc_par_iter.rs"]},{"name":"iter","dirs":[{"name":"collect","files":["consumer.rs","mod.rs"]},{"name":"find_first_last","files":["mod.rs"]},{"name":"plumbing","files":["mod.rs"]}],"files":["chain.rs","chunks.rs","cloned.rs","copied.rs","empty.rs","enumerate.rs","extend.rs","filter.rs","filter_map.rs","find.rs","flat_map.rs","flat_map_iter.rs","flatten.rs","flatten_iter.rs","fold.rs","for_each.rs","from_par_iter.rs","inspect.rs","interleave.rs","interleave_shortest.rs","intersperse.rs","len.rs","map.rs","map_with.rs","mod.rs","multizip.rs","noop.rs","once.rs","panic_fuse.rs","par_bridge.rs","positions.rs","product.rs","reduce.rs","repeat.rs","rev.rs","skip.rs","splitter.rs","step_by.rs","sum.rs","take.rs","try_fold.rs","try_reduce.rs","try_reduce_with.rs","unzip.rs","update.rs","while_some.rs","zip.rs","zip_eq.rs"]},{"name":"slice","files":["mergesort.rs","mod.rs","quicksort.rs"]}],"files":["delegate.rs","lib.rs","math.rs","option.rs","par_either.rs","prelude.rs","private.rs","range.rs","range_inclusive.rs","result.rs","split_producer.rs","str.rs","string.rs","vec.rs"]};
sourcesIndex["rayon_core"] = {"name":"","dirs":[{"name":"compile_fail","files":["mod.rs","quicksort_race1.rs","quicksort_race2.rs","quicksort_race3.rs","rc_return.rs","rc_upvar.rs","scope_join_bad.rs"]},{"name":"join","files":["mod.rs"]},{"name":"scope","files":["mod.rs"]},{"name":"sleep","files":["counters.rs","mod.rs"]},{"name":"spawn","files":["mod.rs"]},{"name":"thread_pool","files":["mod.rs"]}],"files":["job.rs","latch.rs","lib.rs","log.rs","private.rs","registry.rs","unwind.rs","util.rs"]};
sourcesIndex["ryu"] = {"name":"","dirs":[{"name":"buffer","files":["mod.rs"]},{"name":"pretty","files":["exponent.rs","mantissa.rs","mod.rs"]}],"files":["common.rs","d2s.rs","d2s_full_table.rs","d2s_intrinsics.rs","digit_table.rs","f2s.rs","f2s_intrinsics.rs","lib.rs"]};
sourcesIndex["scoped_threadpool"] = {"name":"","files":["lib.rs"]};
sourcesIndex["scopeguard"] = {"name":"","files":["lib.rs"]};
sourcesIndex["serde"] = {"name":"","dirs":[{"name":"de","files":["ignored_any.rs","impls.rs","mod.rs","seed.rs","utf8.rs","value.rs"]},{"name":"private","files":["de.rs","doc.rs","mod.rs","ser.rs","size_hint.rs"]},{"name":"ser","files":["fmt.rs","impls.rs","impossible.rs","mod.rs"]}],"files":["integer128.rs","lib.rs","macros.rs"]};
sourcesIndex["serde_derive"] = {"name":"","dirs":[{"name":"internals","files":["ast.rs","attr.rs","case.rs","check.rs","ctxt.rs","mod.rs","receiver.rs","respan.rs","symbol.rs"]}],"files":["bound.rs","de.rs","dummy.rs","fragment.rs","lib.rs","pretend.rs","ser.rs","try.rs"]};
sourcesIndex["serde_json"] = {"name":"","dirs":[{"name":"features_check","files":["mod.rs"]},{"name":"io","files":["mod.rs"]},{"name":"value","files":["de.rs","from.rs","index.rs","mod.rs","partial_eq.rs","ser.rs"]}],"files":["de.rs","error.rs","iter.rs","lib.rs","macros.rs","map.rs","number.rs","read.rs","ser.rs"]};
sourcesIndex["smallvec"] = {"name":"","files":["lib.rs"]};
sourcesIndex["syn"] = {"name":"","dirs":[{"name":"gen","files":["clone.rs","debug.rs","eq.rs","gen_helper.rs","hash.rs","visit.rs"]}],"files":["attr.rs","await.rs","bigint.rs","buffer.rs","custom_keyword.rs","custom_punctuation.rs","data.rs","derive.rs","discouraged.rs","error.rs","export.rs","expr.rs","ext.rs","file.rs","generics.rs","group.rs","ident.rs","item.rs","lib.rs","lifetime.rs","lit.rs","lookahead.rs","mac.rs","macros.rs","op.rs","parse.rs","parse_macro_input.rs","parse_quote.rs","pat.rs","path.rs","print.rs","punctuated.rs","reserved.rs","sealed.rs","span.rs","spanned.rs","stmt.rs","thread.rs","token.rs","tt.rs","ty.rs","verbatim.rs","whitespace.rs"]};
sourcesIndex["tiff"] = {"name":"","dirs":[{"name":"decoder","files":["ifd.rs","mod.rs","stream.rs"]},{"name":"encoder","files":["colortype.rs","mod.rs","writer.rs"]}],"files":["bytecast.rs","error.rs","lib.rs","tags.rs"]};
sourcesIndex["unicode_xid"] = {"name":"","files":["lib.rs","tables.rs"]};
sourcesIndex["unindent"] = {"name":"","files":["lib.rs"]};
sourcesIndex["wasm_bindgen"] = {"name":"","dirs":[{"name":"cache","files":["intern.rs","mod.rs"]},{"name":"convert","files":["closures.rs","impls.rs","mod.rs","slices.rs","traits.rs"]}],"files":["cast.rs","closure.rs","describe.rs","externref.rs","lib.rs"]};
sourcesIndex["wasm_bindgen_backend"] = {"name":"","files":["ast.rs","codegen.rs","encode.rs","error.rs","lib.rs","util.rs"]};
sourcesIndex["wasm_bindgen_macro"] = {"name":"","files":["lib.rs"]};
sourcesIndex["wasm_bindgen_macro_support"] = {"name":"","files":["lib.rs","parser.rs"]};
sourcesIndex["wasm_bindgen_shared"] = {"name":"","files":["lib.rs"]};
sourcesIndex["wee_alloc"] = {"name":"","files":["const_init.rs","extra_assert.rs","imp_unix.rs","lib.rs","neighbors.rs","size_classes.rs"]};
sourcesIndex["weezl"] = {"name":"","files":["decode.rs","encode.rs","error.rs","lib.rs"]};
createSourceSidebar();
